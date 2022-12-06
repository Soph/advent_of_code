file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

buffer = []
lines[0].split("").each_with_index do |char, i|
  buffer << char
  buffer.shift if buffer.size > 4
  
  if buffer.size == 4 && buffer.uniq.size == 4
    puts "Result1: #{i+1}"
    break
  end
end

buffer = []
lines[0].split("").each_with_index do |char, i|
  buffer << char
  buffer.shift if buffer.size > 14

  if buffer.size == 14 && buffer.uniq.size == 14
    puts "Result2: #{i+1}"
    break
  end
end