require "set"

file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

memory = []
memory_2 = []
memory_2_org = []
groups = []

count = 0
lines[0].chars.each_slice(2) do |file, space|
  memory << [count.to_s] * file.to_i
  memory << ["."] * space.to_i
  groups << [count.to_s] * file.to_i

  memory_2 << [[count] * file.to_i, ["."] * space.to_i]
  memory_2_org << [[count] * file.to_i, ["."] * space.to_i]
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

  # remove old
  before_old = p2_memory[group_index-1]
  after_old = p2_memory[group_index+1]
  if before_old[0] == "." && after_old[0] == "."
    p2_memory.delete_at(group_index+1)
    p2_memory.delete_at(group_index)
    p2_memory.delete_at(group_index-1)
    p2_memory.insert(group_index-1, ["."]*(group.size+before_old.size+after_old.size))
  elsif before_old[0] == "."
    p2_memory.delete_at(group_index)
    p2_memory.delete_at(group_index-1)
    p2_memory.insert(group_index-1, ["."]*(group.size+before_old.size))
  elsif after_old[0] == "."
    p2_memory.delete_at(group_index+1)
    p2_memory.delete_at(group_index)
    p2_memory.insert(group_index, ["."]*(group.size+after_old.size))
  else
    p2_memory[group_index] = ["."]*group.size
  end
  
  p2_memory[first_index] = ["."] * (p2_memory[first_index].size - group.size)
  p2_memory.insert(first_index, group)
end

sum = 0
p2_memory.flatten.each_with_index do |file_id, index|
  sum += file_id.to_i * index
end
puts "Part2: #{sum}"
