require_relative "code_tester.rb"

code = File.read("test_file.rb")
fileReader = CodeTester.new(code)

if fileReader.check_syntax
  puts fileReader.response
else
  puts "Blad"
end
