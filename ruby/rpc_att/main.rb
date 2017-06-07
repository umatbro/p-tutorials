require_relative "code_tester.rb"

#response = `ruby -wc test_file.rb`

code = File.read("test_file.rb")
fileReader = CodeTester.new(code)
#uts fileReader.check_syntax

if fileReader.check_syntax
  puts fileReader.response
else
  puts "Blad"
end
