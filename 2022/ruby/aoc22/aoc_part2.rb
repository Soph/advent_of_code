require "set"
file = File.read(ARGV[0])

Point = Struct.new(:x, :y) do
  def to_s
    #"(#{x+1},#{y+1})"
    "(#{x},#{y})"
  end

  def move(dx, dy)
    Point.new(x + dx, y + dy)
  end
end

Move = Struct.new(:steps, :turn) do
  def to_s
    "(#{steps}#{turn})"
  end
end

parts = file.split("\n\n")
movements = parts[1].scan(/\d+\w?/).map{|move| parsed = move.scan(/\d+|R|L/); Move.new(parsed[0].to_i, parsed[1])}

$dimensions_x = []
$dimensions_y = []
walls = Set.new

lines = parts[0].split("\n")
lines.each_with_index do |line, y|
  min_x = line.length
  max_x = 0
  line.chars.each_with_index do |item, x|
    next if item == " "
    min_x = x if x < min_x
    max_x = x if x > max_x
      
    walls << Point.new(x, y) if item == "#"
    $dimensions_y[x] ||= [lines.size, 0]
    $dimensions_y[x][0] = y if y < $dimensions_y[x][0]
    $dimensions_y[x][1] = y if y > $dimensions_y[x][1]  
  end
  $dimensions_x << [min_x, max_x]
end
$cur = Point.new($dimensions_x[0][0], 0)
directions = [[1,0], [0,1], [-1,0], [0,-1]]
direction_index = 0
#               right,    down,   left,  up
# directions = [[1,0], [0,1], [-1,0], [0,-1]]

# example
# $translates = {
#   :top => [
#     {
#       :direction => 1,
#       :point => -> (p) { Point.new(8 + $width - 1 - (p.x % $width), 0) }
#     },
#     {
#       :direction => 0,
#       :point => -> (p) { Point.new(8, (p.x % $width)) }
#     },
#     {
#       :direction => 1,
#       :point => -> (p) { Point.new(0 + $width - 1 - (p.x % $width), 4) }
#     }
#   ],
#   :bottom => [
#     {
#       :direction => 3,
#       :point => -> (p) { Point.new(8 + $width - 1 - (p.x % $width), 11) }
#     },
#     {
#       :direction => 0,
#       :point => -> (p) { Point.new($width - 1 - (p.y % $width), 4) }
#     },
#     {
#       :direction => 3,
#       :point => -> (p) { Point.new($width - 1 - (p.x % $width), 7) }
#     },
#     {
#       :direction => 0,
#       :point => -> (p) { Point.new(0, 4 + $width - 1 - (p.x % $width)) }
#     }
#   ],
#   :right => [
#     {
#       :direction => 2,
#       :point => -> (p) { Point.new(11, $width - 1 - (p.y % $width))}
#     },
#     {
#       :direction => 1,
#       :point => -> (p) { Point.new(p.x + 1 + (p.y % $width), 8)}
#     },
#     {
#       :direction => 2,
#       :point => -> (p) { Point.new(p.x - $width - 1, $width - 1 - p.x % $width)}
#     },
#   ],
#   :left => [
#     {
#       :direction => 1,
#       :point => -> (p) { Point.new(4 + (p.y % $width), 3)}
#     },
#     {
#       :direction => 3,
#       :point => -> (p) { Point.new(11 + ($width - 1 - p.y % $width), 3 * $width - 1)}
#     },
#     {
#       :direction => 3,
#       :point => -> (p) { Point.new($width + ($width - p.y % $width), 7)}
#     },    
#   ]
# }
# $width = 4

#               right,    down,   left,  up
# directions = [[1,0], [0,1], [-1,0], [0,-1]]

# example
$translates = {
  :top => [
    {
      :direction => 0,
      :point => -> (p) { Point.new($width, $width + (p.x % $width)) }
    },
    {
      :direction => 0,
      :point => -> (p) { Point.new(0, $width * 3 + (p.x % $width)) }
    },
    {
      :direction => 3,
      :point => -> (p) { Point.new(0 + (p.x % $width), 4*$width-1) }
    }
  ],
  :bottom => [
    {
      :direction => 1,
      :point => -> (p) { Point.new($width * 2 + p.x, 0) }
    },
    {
      :direction => 2,
      :point => -> (p) { Point.new($width - 1, 3 * $width + (p.x % $width)) }
    },
    {
      :direction => 2,
      :point => -> (p) { Point.new(2*$width - 1, $width + (p.x % $width)) }
    }
  ],
  :right => [
    {
      :direction => 2,
      :point => -> (p) { Point.new(2 * $width - 1, 3 * $width - 1 - (p.y % $width))}
    },
    {
      :direction => 3,
      :point => -> (p) { Point.new(p.x + (p.y % $width), $width - 1)}
    },
    {
      :direction => 2,
      :point => -> (p) { Point.new(3*$width-1, $width - 1 - p.y % $width)}
    },
    {
      :direction => 3,
      :point => -> (p) { Point.new($width + p.y % $width, 3*$width - 1)}
    },
  ],
  :left => [
    {
      :direction => 0,
      :point => -> (p) { Point.new(0, 3 * $width - 1 - p.y % $width)}
    },
    {
      :direction => 1,
      :point => -> (p) { Point.new(0 + p.y % $width, 2 * $width)}
    },
    {
      :direction => 0,
      :point => -> (p) { Point.new($width, $width - 1 - p.y % $width)}
    },
    {
      :direction => 1,
      :point => -> (p) { Point.new($width + p.y % $width, 0)}
    },
  ]
}
$width = 50
def translate(pos, direction_index)
  translate = nil
  if direction_index % 2 == 0 # horizontal
    if pos.x < $dimensions_x[pos.y][0] # moving out left
      translate = $translates[:left][pos.y / $width] 
    elsif pos.x > $dimensions_x[pos.y][1] # moving out right
      translate = $translates[:right][pos.y / $width]
    end
  else
    #puts "vertical: #{pos}"
    if pos.y < $dimensions_y[pos.x][0] # moving out top
      #puts "moving out top"
      translate = $translates[:top][pos.x / $width] 
    elsif pos.y > $dimensions_y[pos.x][1] # moving out bottom
      #puts "moving out bottom"
      translate = $translates[:bottom][pos.x / $width] 
    end
  end
  translate
end

$cur = Point.new($dimensions_x[0][0], 0)
             # right,   up,   left,  down
directions = [[1,0], [0,1], [-1,0], [0,-1]]
direction_index = 0
movements.each do |move|
  puts "#{$cur}: #{move} #{directions[direction_index]}"
  move.steps.times do
    new_pos = $cur.move(*directions[direction_index])
    #puts "Check translation: #{$cur}"
    translate = translate(new_pos, direction_index)
    if translate
      puts "Change Square: #{translate}"
      new_pos = translate[:point].call(new_pos)
      puts "Move: #{$cur} -> #{new_pos} (#{direction_index})"
      if walls.include?(new_pos)
        #puts "#{$cur}: Hit a wall at #{new_pos}"
        break
      end  
      direction_index = translate[:direction]
    elsif walls.include?(new_pos)
      #puts "#{$cur}: Hit a wall at #{new_pos}"
      break
    end
    #puts "Move: #{$cur} -> #{new_pos}"
    $cur = new_pos
  end
  if move.turn == "L"
    #puts "#{$cur}: turn left (#{direction_index})"
    if direction_index == 0
      direction_index = (directions.size - 1)
    else
      direction_index = (direction_index - 1) % (directions.size)
    end
  elsif move.turn == "R"
    #puts "#{$cur}: turn right (#{direction_index})"
    direction_index = (direction_index + 1) % (directions.size)
  end
  #puts "#{direction_index}"
  #sleep 5
end

puts $cur.to_s
result = 1000 * ($cur.y + 1) + 4 * ($cur.x + 1) + direction_index
puts "Result2: #{result}"
