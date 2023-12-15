file = File.open(ARGV[0])
data = file.readlines.map(&:chomp).first.split(",")

def hash(string)
  value = 0
  string.chars.each do |char|
    value += char.ord
    value *= 17 
    value %= 256
  end
  value
end

sum = 0
data.each do |string|
  value = hash(string)
  sum += value
end

puts "Part1: #{sum}"

boxes = {}
data.each do |string|
  i = string.index(/[\=\-]/)
  label = string[0...i]
  box = hash(label)
  boxes[box] ||= []
  operation = string[i...i+1]
  index = boxes[box].find_index{|lens| lens[0] == label }
  if operation == '-'
    boxes[box].delete_at(index) unless index.nil?
  else # = 
    number = string[i+1..].to_i
    if index.nil?
      boxes[box] << [label, number]
    else
      boxes[box][index] = [label, number]
    end
  end
  #puts boxes.inspect
end

sum = 0
boxes.each do |box, lenses|
  lenses.each_with_index do |lens, i|
    sum += (box + 1) * (i+1) * lens[1]
  end
end

puts "Part2: #{sum}"
