file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

def to_name(i)
  case i
  when 1
    'one'
  when 2
    'two'
  when 3
    'three'
  when 4
    'four'
  when 5
    'five'
  when 6
    'six'
  when 7
    'seven'
  when 8
    'eight'
  when 9
    'nine'
  end
end

sum = 0
lines.each do |line|
  (1..9).each do |n|
    line = line.gsub(to_name(n), to_name(n) + n.to_s + to_name(n))
  end
  #puts line
  numbers = line.gsub(/[a-zA-Z]/, "")
  sum += (numbers.chars[0] + numbers.chars[-1]).to_i
end

puts sum