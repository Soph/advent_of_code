file = File.open(ARGV[0])
data = file.readlines.map(&:chomp)[0]

muls = data.scan(/mul\((\d*)\,(\d*)\)/)
result = muls.sum do |mul|
  mul[0].to_i * mul[1].to_i
end

puts "Part1: #{result}"

muls = data.scan(/mul\((\d*)\,(\d*)\)|(do\(\))|(don\'t\(\))/)
enabled = true
result = muls.sum do |mul|
  if mul[0].nil?
    if mul[2] == 'do()'
      enabled = true
    else
      enabled = false
    end
    0
  elsif enabled
    mul[0].to_i * mul[1].to_i 
  else
    0
  end
end

puts "Part2: #{result}"

