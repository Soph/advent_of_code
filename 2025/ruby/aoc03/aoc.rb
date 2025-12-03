file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

maxes = []
lines.each do |line|
  variants = []
  numbers = line.split('')
  numbers.each_with_index do |first, i|
    numbers[i + 1..].each do |second|
      variants << (first + second).to_i
    end
  end
  maxes << variants.max
end

puts maxes.inspect
puts maxes.sum

LENGTH = 12
def find_max(numbers, remaining)
  max = -1
  pos = -1
  numbers.each_with_index do |number, i|
    break if numbers.length - i < remaining

    if number > max
      max = number
      pos = i
    end
  end
  [max, pos]
end
maxes = []
lines.each do |line|
  numbers = line.split('').map(&:to_i)
  max_string = []
  i = 0
  loop do
    remaining = LENGTH - max_string.size
    new_max, new_i = find_max(numbers[i..], remaining)
    i += new_i + 1
    max_string << new_max if new_max > -1
    break if max_string.size == LENGTH
  end
  maxes << max_string.join.to_i
end

puts maxes.inspect
puts maxes.sum
