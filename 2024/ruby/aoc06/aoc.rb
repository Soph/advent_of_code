require "set"

file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

@width = lines[0].length
@height = lines.count

obstacles = Set.new
directions = [[0,-1], [1,0], [0,1], [-1,0]]

initial_guard_position = nil
guard_direction = [0,-1]
watched = Set.new

lines.each_with_index do |line, y|
  line.split("").each_with_index do |position, x|
    if position == '^'
      initial_guard_position = [x,y]
    elsif position == '#'
      obstacles << [x,y]
    end
  end
end

guard_position = initial_guard_position.clone

watched << guard_position.clone

def out_of_bounds?(guard_position)
  return true if guard_position[0] < 0
  return true if guard_position[1] < 0
  return true if guard_position[0] >= @width
  return true if guard_position[1] >= @height

  false
end

def print_maze(obstacles, watched)
  (0...@height).each do |y|
    (0...@width).each do |x|
      if obstacles.include?([x,y])
        putc "#"
      elsif watched.include?([x,y])
        putc "X"
      else
        putc "."
      end
    end
    puts
  end
end

while !out_of_bounds?(guard_position) do
  new_position = [guard_position[0]+guard_direction[0], guard_position[1]+guard_direction[1]]
  if obstacles.include?(new_position) # turn right
    guard_direction = directions[(directions.find_index(guard_direction)+1)%4]
  else
    watched << new_position.clone
    guard_position = new_position
  end
end

print_maze(obstacles, watched)
puts "Part1: #{watched.size - 1}" # we add out of bounds position

loops = 0
watched.each do |new_obstacle|
  p2_obstacles = obstacles.clone
  p2_obstacles << new_obstacle.clone
  guard_position = initial_guard_position.clone
  guard_direction = [0,-1]

  p2_watched = {}
  while !out_of_bounds?(guard_position) do
    new_position = [guard_position[0]+guard_direction[0], guard_position[1]+guard_direction[1]]
    if p2_obstacles.include?(new_position) # turn right
      guard_direction = directions[(directions.find_index(guard_direction)+1)%4]
    else
      p2_watched[new_position] ||= Set.new
      if p2_watched[new_position].include?(guard_direction)
        loops += 1
        break
      else
        p2_watched[new_position] << guard_direction.clone
        guard_position = new_position
      end
    end
  end
end

puts "Part2: #{loops}"