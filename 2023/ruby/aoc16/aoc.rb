require "Set"

file = File.open(ARGV[0])
grid = file.readlines.map(&:chomp).map(&:chars)

mapping = {
  '.' => {
    '^' => [[[0,-1],"^"]],
    '>' => [[[1,0],">"]],
    'v' => [[[0,1],"v"]],
    '<' => [[[-1,0],"<"]],
  },
  '|' => {
    '^' => [[[0,-1],"^"]],
    '>' => [[[0,-1],"^"], [[0,1],"v"]],
    'v' => [[[0,1],"v"]],
    '<' => [[[0,-1],"^"], [[0,1],"v"]],
  },
  '-' => {
    '^' => [[[1,0],">"], [[-1,0],"<"]],
    '>' => [[[1,0],">"]],
    'v' => [[[1,0],">"], [[-1,0],"<"]],
    '<' => [[[-1,0],"<"]],
  },
  '/' => {
    '^' => [[[1,0],">"]],
    '>' => [[[0,-1],"^"]],
    'v' => [[[-1,0],"<"]],
    '<' => [[[0,1],"v"]],
  },
  '\\' => {
    '^' => [[[-1,0],"<"]],
    '>' => [[[0,1],"v"]],
    'v' => [[[1,0],">"]],
    '<' => [[[0,-1],"^"]],
  }
}

beams = [[[0,0],">"]]
done = Set.new
heated = Set.new
heated << [0,0]
while !beams.empty?
  beam = beams.shift
  
  mapping[grid[beam[0][1]][beam[0][0]]][beam[1]].each do |new_beam|
    pos = [beam[0][0] + new_beam[0][0],beam[0][1] + new_beam[0][1]]
    puts "#{beam[1]} #{grid[beam[0][1]][beam[0][0]]} #{pos.inspect} #{new_beam[1]}"
    next if pos[0] < 0 || pos[0] >= grid[0].size # out of bounds hori
    next if pos[1] < 0 || pos[1] >= grid.size # out of bounds verti
    next if done.include?([pos, new_beam[1]])
    heated << pos
    beams.push([pos, new_beam[1]])
  end
  done.add(beam.map(&:clone))
end

puts "Part 1: #{heated.size}"

max = 0
variants = []
(0...grid.size).each do |i|
  variants << [[0,i], '>']
  variants << [[grid.size-1,i], '<']
end
(0...grid[0].size).each do |i|
  variants << [[i,0], 'v']
  variants << [[i,grid[0].size-1], '^']
end

max = 0
variants.each do |variant|
  beams = [variant]
  done = Set.new
  heated = Set.new
  heated << variant[0]
  while !beams.empty?
    beam = beams.shift
    
    mapping[grid[beam[0][1]][beam[0][0]]][beam[1]].each do |new_beam|
      pos = [beam[0][0] + new_beam[0][0],beam[0][1] + new_beam[0][1]]
      #puts "#{beam[1]} #{grid[beam[0][1]][beam[0][0]]} #{pos.inspect} #{new_beam[1]}"
      next if pos[0] < 0 || pos[0] >= grid[0].size # out of bounds hori
      next if pos[1] < 0 || pos[1] >= grid.size # out of bounds verti
      next if done.include?([pos, new_beam[1]])
      heated << pos
      beams.push([pos, new_beam[1]])
    end
    done.add(beam.map(&:clone))
  end
  #puts "#{variant}: #{heated.size}"
  max = heated.size if max < heated.size
end
puts "Part 2: #{max}"