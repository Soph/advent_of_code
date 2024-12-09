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
  p2_memory.each_slice(2).with_index do |data, free, index|
    next if free.size < group.size

    source_index = p2_memory.index(group)
    # replace source with free
    p2_memory[source_index] = []
    p2_memory[source_index] += ["."] * group.size

    if free.size == group.size
      free = []
      p2_memory.insert(index*2+1, [group])
    else

    end
  end
end


puts memory_2.inspect
(memory_2.size-1).downto(0).each do |index|
  from_memory = memory_2_org[index]
  #puts from_memory.inspect
  memory_2[0..index].each_with_index do |data, data_index|
    from_data_size = from_memory[0].size
    to_free_size = data[1].size
    next if to_free_size < from_data_size

    # copy data to new position
    data[0] += from_memory[0]
    # remove free space at new position
    data[1] = ["."] * (to_free_size - from_data_size)

    # update old position
    #puts "#{from_memory[1].size} + #{from_data_size}"
    memory_2[index] = [memory_2[index][0]-from_memory[0], memory_2[index][1]]
    memory_2.insert(index, [[], ["."] * from_data_size])
    #puts memory_2.flatten.join.inspect
    break
  end
end

puts memory_2.flatten.join.inspect
sum = 0
memory_2.flatten.join.chars.each_with_index do |file_id, index|
  sum += file_id.to_i * index
end
puts "Part1: #{sum}"

# files_2 = files.clone.flatten
# new_memory = []
# spaces.each_with_index do |free, index|
#   free.times do |i|
#     files[index] << files_2.pop
#   end
# end

# puts files.inspect


# diskmap = lines[0].chars
# diskmap_new = []
# max_file_id = diskmap.size / 2

# diskmap.each_slice(2) do |pos, index|
#   if index % 2 == 0
#     diskmap_new << ((index / 2).to_s * pos.to_i).chars
#   else
#     while count > 0
#       if diskmap[max_file_id * 2].to_i > count
#         diskmap_new 

#     count = pos.to_i
#     if diskmap[max_file_id * 2].to_i > count

#     else

#     end
#     while count > 0
#       diskmap[end_index] 
#     end
#     #new_end_index = end_index - pos.to_i
#     #diskmap << diskmap[new_end_index, end_index]
#   end
# end


