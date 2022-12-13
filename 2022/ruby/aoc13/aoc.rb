file = File.read(ARGV[0])
pairs = file.split("\n\n")

def compare(left, right)
  puts "#{left} vs #{right}"
  left.each_with_index do |l, n|
    if right[n].nil?
      puts "right has less items -> invalid"
      return -1
    end
    puts "#{l} vs #{right[n]}"
    #sleep(1)
    if right[n].class == l.class
      if l.is_a?(Array)
        res = compare(l, right[n])
        return -1 if res == -1
        return 1 if res == 1
      else
        if l > right[n]
          puts "left is bigger -> invalid"
          return -1
        end
        if l < right[n]
          puts "left is smaller -> valid"
          return 1
        end
      end
    else
      r = right[n]
      if l.is_a?(Array)
        r = [r]
      else
        l = [l]
      end
      res = compare(l, r)
      return -1 if res == -1
      return 1 if res == 1
    end
  end
  #puts "Nothing wrong so far, valid"
  if right.size > left.size
    puts "left run out of items, right still has -> valid"
    return 1
  end
  return 0
end

correct = []
all_packets = []
pairs.each_with_index do |pair, i|
  parts = pair.split("\n")
  left = eval(parts[0])
  right = eval(parts[1])
  all_packets << left
  all_packets << right
  puts "==== Pair #{i+1} ===="
  if compare(left, right) == 1
    correct << i+1
    puts "#{i+1} is valid"
  else
    puts "#{i+1} is not valid"
  end
  #sleep(5)
end

puts "Valid: #{correct.inspect}"
puts "Result1: #{correct.sum}"

divider_a = [[2]]
divider_b = [[6]]
all_packets << divider_a
all_packets << divider_b

sorted = all_packets.sort{|a,b| compare(a,b) }.reverse
divider_indexes = []

sorted.each_with_index do |packet, i|
  divider_indexes << i+1 if packet == divider_a || packet == divider_b
end

puts divider_indexes.inject(:*)