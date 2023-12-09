file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

all = lines.map{ |line| line.split(" ").map(&:to_i) }

sum1 = 0
sum2 = 0
all.each do |numbers|
  stacks = []
  stack = numbers
  while true do
    stacks << stack
    current = stack
    stack = []
    current.each_with_index do |number, i|
      break if i+1 == current.size
      stack << current[i+1] - number 
    end
    break if stack.uniq == [0]
  end
  # puts stacks.inspect
  (stacks.size-2).downto(0).each do |i|
    stacks[i] << stacks[i][-1] + stacks[i+1][-1] 
    stacks[i] = stacks[i].unshift(stacks[i][0]  - stacks[i+1][0] )
  end
  # puts stacks[0][-1]
  # puts stacks[0][0]
  sum1 += stacks[0][-1]
  sum2 += stacks[0][0]
end

puts "Part1: #{sum1}"
puts "Part2: #{sum2}"