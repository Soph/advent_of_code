file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

sum = 0
lines.each do |line|
  numbers = line.gsub(/[a-zA-Z]/, "")
  sum += (numbers.chars[0] + numbers.chars[-1]).to_i
end

puts sum
