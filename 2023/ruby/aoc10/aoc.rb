file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

maze = lines.map { |line| line.chars }

start = []

maze.each_with_index do |row, y|
  row.each_with_index do |pos, x|
    start = [x,y] if pos == "S"
  end
end

north = ['7', 'F', '|']
south = ['L', 'J', '|']
east = ['7', 'J', '-']
west = ['L', 'F', '-']

checks = {
  'S' => {
    [0,1] => south,
    [0,-1] => north,
    [1,0] => east,
    [-1,0] => west
  },
  'L' => {
    [0,-1] => north,
    [1,0] => east
  },
  'J' => {
    [0,-1] => north,
    [-1,0] => west
  },
  '|' => {
    [0,-1] => north,
    [0,1] => south
  },
  '-' => {
    [1,0] => east,
    [-1,0] => west
  },
  '7' => {
    [-1,0] => west,
    [0,1] => south
  },
  'F' => {
    [0,1] => south,
    [1,0] => east
  }
}

visited = {start => 0}
possible = [start]

while true do
  new_possible = []
  possible.each do |pos|
    local_checks = checks[maze[pos[1]][pos[0]]]
    local_checks.keys.each do |delta|
      distance = visited[pos] + 1
      new_pos = [pos[0] + delta[0], pos[1] + delta[1]]
      next if new_pos[0] < 0 || new_pos[1] < 0
      next if maze[new_pos[1]][new_pos[0]].nil?
      next if maze[new_pos[1]][new_pos[0]] == '.'
      next if !local_checks[delta].include?(maze[new_pos[1]][new_pos[0]])

      visited[new_pos] ||= 99999999999
      if visited[new_pos] > distance
        visited[new_pos] = distance
        new_possible << [new_pos[0],new_pos[1]]
      end
    end
  end 
  possible = new_possible
  break if possible == []
end

puts "Part1: #{visited.values.max}"

inside_tiles = []
maze.each_with_index do |row, y|
  inside = false
  last_open = nil
  row.each_with_index do |pos, x|
    pos = '|' if pos == 'S' # hack for real input
    if visited[[x,y]].nil?
      inside_tiles << [x,y] if inside
    elsif pos == '|'
      inside = !inside
    elsif pos == 'L' || pos == 'F'
      last_open = pos
    elsif pos == 'J' && last_open == 'F'
      inside = !inside 
    elsif pos == '7' && last_open == 'L'
      inside = !inside 
    end
  end
end

puts "Part2: #{inside_tiles.size}"

# maze.each_with_index do |row, y|
#   row.each_with_index do |pos, x|
#     if inside_tiles.include?([x,y])
#       putc "X"
#     elsif pos == '.'
#       putc "O"
#     else
#       putc pos
#     end
#   end
#   puts ""
# end