require "set"

file = File.open(ARGV[0])
data = file.readlines.map(&:chomp).join("\n").split("\n\n")

lines = data[0].split("\n")
@width = lines[0].length
@height = lines.count

def print_warehouse(boxes, walls, bot)
  @height.times do |y|
    @width.times do |x|
      if boxes.include?([x,y])
        putc "O"
      elsif walls.include?([x,y])
        putc "#"
      elsif bot == [x,y]
        putc "@"
      else
        putc "."
      end
    end
    puts
  end
end

walls = Set.new
boxes = Set.new
bot = nil

lines.each_with_index do |line, y|
  line.split("").each_with_index do |position, x|
    next if position == '.'
    if position == '#'
      walls << [x,y]
    elsif position == 'O'
      boxes << [x,y]
    else
      bot = [x,y]
    end
  end
end

moves = data[1].split("\n").join.split("")

dirmap = {
  ">" => [1,0],
  "^" => [0,-1],
  "v" => [0,1],
  "<" => [-1,0]
}
moves.each do |move|
  dir = dirmap[move]
  new_pos = [bot[0]+dir[0], bot[1]+dir[1]]
  next if walls.include?(new_pos)
  if boxes.include?(new_pos)
    # figure out push
    push_boxes = Set.new
    new_pos1 = new_pos
    while boxes.include?(new_pos1)
      push_boxes << new_pos1
      new_pos1 = [new_pos1[0]+dir[0], new_pos1[1]+dir[1]]
    end
    if walls.include?(new_pos1)
      # only boxes and then a wall, nothing we can do
      next
    else
      bot = new_pos
      boxes -= push_boxes
      push_boxes.each do |box|
        boxes << [box[0]+dir[0], box[1]+dir[1]]
      end
    end
  else
    # nothing in the way, let's move here
    bot = new_pos
  end
end

print_warehouse(boxes, walls, bot)

result = 0
boxes.each do |box|
  result += box[0] + box[1]*100
end

puts "Part1: #{result}"
