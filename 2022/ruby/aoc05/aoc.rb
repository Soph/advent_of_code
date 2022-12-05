file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

stacks = []
moves = []
first = true
lines.each_with_index do |line|
  if line == ""
    first = false
    next
  end

  if first
    y = 0
    line.chars.each_slice(4) do |crate|
      if crate[0] != " "
        stacks[y] ||= []

        stacks[y].unshift(crate[1])
      end
      y += 1
    end
  else
    moves << line
  end
end
stacks_second = stacks.map(&:clone)

moves.each do |move|
  parsed = /move (\d*) from (\d*) to (\d*)/.match(move)
  parsed[1].to_i.times do
    crate = stacks[parsed[2].to_i-1].pop()
    stacks[parsed[3].to_i-1].push(crate)
  end
end

puts "Result1: #{stacks.map {|stack| stack.last }.join()}"

stacks = stacks_second

i = 0
moves.each do |move|
  parsed = /move (\d*) from (\d*) to (\d*)/.match(move)
  crate = stacks[parsed[2].to_i-1].pop(parsed[1].to_i)
  stacks[parsed[3].to_i-1].push(*crate)
end

puts "Result2: #{stacks.map {|stack| stack.last(1) }.join()}"