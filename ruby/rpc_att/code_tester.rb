class CodeTester
  attr_accessor :code
  attr_reader :response
  def initialize(code=nil)
    @code = code
    @syntax = false; # syntax variable tells if code provided is correct
  end

  def check_syntax
    if @code != nil
      file = File.new("temp.rb","w")
      file.puts(@code)
      file.close
      response = `ruby -c temp.rb`

      # check if @code syntax is correct
      if response == "Syntax OK\n"
        @syntax = true
      else
        @syntax = false
      end
    else
      puts "No code provided"
      @syntax = false
    end

    # save code output if its correct
    if @syntax
      @response = `ruby temp.rb`
    end
    `rm temp.rb`
    return @syntax
  end
end
