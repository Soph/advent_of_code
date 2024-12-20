require "set"

file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

memory = []
groups = []

count = 0
lines[0].chars.each_slice(2) do |file, space|
  memory << [count.to_s] * file.to_i
  memory << ["."] * space.to_i
  groups << [count.to_s] * file.to_i
  count += 1
end

p1_memory = memory.flatten.clone
p1_memory.each_with_index do |item, index|
  next unless item == "."

  while true do
    new_item = p1_memory.pop
    break if p1_memory.size == index
    if new_item != "." 
      p1_memory[index] = new_item
      break
    end
  end
end

sum = 0
p1_memory.each_with_index do |file_id, index|
  sum += file_id.to_i * index
end
puts "Part1: #{sum}"

p2_memory = memory.map(&:clone)
groups.reverse.each do |group|
  group_index = p2_memory.index(group)
  first_index = p2_memory.index { |data| data[0] == "." && data.size >= group.size }

  # is this more forward?
  next if first_index.nil? || group_index <= first_index

  p2_memory[group_index] = ["."] * group.size
  p2_memory[first_index] = ["."] * (p2_memory[first_index].size - group.size)
  p2_memory.insert(first_index, group)
end

sum = 0
p2_memory.flatten.each_with_index do |file_id, index|
  sum += file_id.to_i * index
end
puts "Part2: #{sum}"
