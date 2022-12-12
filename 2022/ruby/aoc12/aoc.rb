file = File.open(ARGV[0])
grid = file.readlines.map(&:chomp).map(&:chars)

Point = Struct.new(:x, :y) do
  def to_s
    "(#{x},#{y})"
  end
end

start_pos = nil
end_pos = nil
a_positions = []
grid.size.times do |y|
  grid[0].size.times do |x|
    if grid[y][x] == 'S'
      start_pos = Point.new(x, y)
    end
    if grid[y][x] == 'E'
      end_pos = Point.new(x, y)
    end
    if grid[y][x] == 'a'
      a_positions << Point.new(x, y)
    end
  end
end

puts "Start: #{start_pos} End: #{end_pos}"

path = []

def find_path(start_pos, end_pos, grid)
  puts "Start: #{start_pos}"
  search_matrix = [
    [-1, 0],
    [0, 1],
    [0, -1],
    [1, 0]
  ]  
  ways = { start_pos => 0}
  visited = { }

  while ways.any?
    current_pos, moves = ways.min_by { |_, letter| letter }
    ways.delete(current_pos)

    visited[current_pos] = moves
    current_letter = grid[current_pos.y][current_pos.x]
    # puts "current pos: #{current_pos} #{current_letter}"
    search_matrix.each do |direction|
      new_pos = Point.new(current_pos.x + direction[0], current_pos.y + direction[1])
      next if new_pos.x >= grid[0].size || new_pos.y >= grid.size || new_pos.x < 0 || new_pos.y < 0
      next if visited[new_pos]
      next if grid[new_pos.y][new_pos.x] != current_letter && grid[new_pos.y][new_pos.x] > current_letter.next
      ways[new_pos] = moves + 1
    end
  end

  visited[end_pos]
end

grid[start_pos[1]][start_pos[0]] = "a"
grid[end_pos[1]][end_pos[0]] = "z"
path = find_path(start_pos, end_pos, grid)
puts "Result1: #{path}"

puts "Result2: #{a_positions.map {|start_pos| find_path(start_pos, end_pos, grid)}.compact.sort.first}"