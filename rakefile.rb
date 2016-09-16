
# The location of the compiled TivaWare Peripheral Driver Library.
driverlib = "lib/TivaWare/driverlib/gcc/libdriver.a"
binary = "target/cortex-m4f/debug/rust-tiva-blinky"

task :default => :load

task :build => "#{binary}"

# The debug build requires driverlib to be built first.
desc "Build the project in debug mode"
task "#{binary}" => "#{driverlib}" do
    puts "Building rust app with xargo..."
    sh "xargo build --target cortex-m4f"
end

desc "Flash the applicaion onto the board"
task :load => "#{binary}" do
    puts "Flashing with openocd..."
    sh "openocd -f board/ek-tm4c123gxl.cfg -c \"program #{binary} verify reset exit\""
end

desc "Start openocd"
task :ocd do
    puts "Starting openocd..."
    sh "openocd -f board/ek-tm4c123gxl.cfg"
end

desc "Start a debug session with gdb"
task :gdb => "#{binary}" do
    puts "Starting gdb..."
    sh "arm-none-eabi-gdb #{binary}" +
        %[ --ex "target remote :3333"] +
        %[ --ex load] +
        %[ --ex "tui enable"]
end

# Build instructions for the TivaWare Peripheral Driver Library (i.e. "driverlib").
namespace :driverlib do

    driverlib_src_path = "lib/TivaWare/driverlib"

    desc "Build TivaWare Peripheral Driver Library"
    task :build => "#{driverlib}"

    file "#{driverlib}" do |task| 
        puts "Building #{File.basename(task.name)}..."
        chdir("#{driverlib_src_path}") do
            sh "make"
        end
    end

    desc "Clean TivaWare Peripheral Driver Library"
    task :clean do |task|
        puts "Cleaning #{File.basename(task.name)}..."
        chdir("#{driverlib_src_path}") do
            sh "make clean"
        end
    end

end