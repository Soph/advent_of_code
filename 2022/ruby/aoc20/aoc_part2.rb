require "set"
file = File.open(ARGV[0])
numbers = file.readlines.map(&:chomp).map(&:to_i)
trace = false
encryption_key = 811589153

Item = Struct.new(:value, :nex, :pre) do
  def to_s
    value
    #"(#{pre.value}<-#{value}->#{nex.value})"
  end
end

def print_list(first, items, numbers)
  first = items.select{|item| item.value == 0}.first
  result = []
  cur = first
  (0..numbers.size-1).each do
    result << cur.value
    cur = cur.nex
  end
  
  puts result.inspect
end

items = []
(0..numbers.size-1).each do |i|
  pre = items.last
  cur = Item.new(numbers[i] * encryption_key, nil, pre)
  items << cur
  pre.nex = cur if pre
end
items.last.nex = items[0]
items[0].pre = items.last
#puts items.select {|item| item.pre.nil? || item.nex.nil?}.size

print_list(items[0], items, numbers) if trace
first = items[0]
10.times do |i|
  items.each do |item|
    move = item.value.abs % (numbers.size-1)
    right = item.value > 0  
    next if move == 0
    if right
      (0..move-1).each do
        # A <> B <> C <> D
        # move B by one right
        # swap place with next
        # get old
        old_nex = item.nex # C
        old_pre = item.pre # A
        new_nex = old_nex.nex # D
        new_pre = old_nex # C

        # move item
        item.nex = new_nex # B <> D
        new_nex.pre = item # B <> D
        item.pre = new_pre # C <> B
        new_pre.nex = item # C <> B
        
        # close gap
        old_nex.pre = old_pre # A <> C
        old_pre.nex = old_nex # A <> C
        if first == item
          first = old_nex
        end
      end
    else
      (0..move-1).each do
        # A <> B <> C <> D
        # move B by one left      
        # swap place with prev
        # get old
        old_nex = item.nex    # C
        old_pre = item.pre    # A
        new_nex = old_pre     # A
        new_pre = old_pre.pre # D

        # move item
        item.nex = new_nex # B <> A
        new_nex.pre = item # B <> A
        item.pre = new_pre # D <> B
        new_pre.nex = item # D <> B
        
        # close gap
        old_pre.nex = old_nex # A <> C 
        old_nex.pre = old_pre # A <> C
        if first == item
          first = old_nex
        end
      end
    end
    #puts "Moving: #{item.to_s} between #{item.pre.to_s} and #{item.nex.to_s}" if trace

  end
  puts "Round: #{i}"
  print_list(first, items, numbers) if trace
end

start = items.select{|item| item.value == 0}.first
cur = start
results = []
(0..3000).each do |i|
  if i % 1000 == 0
    results << cur.value
    puts results.inspect
  end
  cur = cur.nex
end

puts results.inspect
puts "Result: #{results.sum}"