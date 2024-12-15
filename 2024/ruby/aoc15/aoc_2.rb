require "set"

file = File.open(ARGV[0])
data = file.readlines.map(&:chomp).join("\n").split("\n\n")

lines = data[0].split("\n")
@width = lines[0].length*2
@height = lines.count

def print_warehouse(boxes, walls, bot)
  @height.times do |y|
    @width.times do |x|
      if boxes.map{|box| box[0]}.include?([x,y])
        putc "["
      elsif boxes.map{|box| box[1]}.include?([x,y])
        putc "]"
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
      walls << [x*2,y]
      walls << [x*2+1,y]
    elsif position == 'O'
      boxes << [[x*2,y], [x*2+1,y]]
    else
      bot = [x*2,y]
    end
  end
end

print_warehouse(boxes, walls, bot)

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
  if boxes.select{|box| box[0] == new_pos || box[1] == new_pos }.size > 0
    # figure out push
    push_boxes = Set.new
    check_positions = Set.new
    check_positions << new_pos
    wall_found = false
    can_move = false
    while !wall_found && !can_move
      new_check_pos = Set.new
      new_push_boxes = Set.new
      check_positions.each do |check_pos|
        if walls.include?(check_pos)
          wall_found = true
          break
        else
          new_push_boxes += boxes.select{ |box| box[0] == check_pos || box[1] == check_pos }
          new_push_boxes.each do |box|
            if move == '>'
              new_check_pos << [box[1][0]+dir[0], box[1][1]+dir[1]]
            elsif move == '<'
              new_check_pos << [box[0][0]+dir[0], box[0][1]+dir[1]]
            else
              new_check_pos << [box[1][0]+dir[0], box[1][1]+dir[1]]
              new_check_pos << [box[0][0]+dir[0], box[0][1]+dir[1]]
            end
          end
        end        
      end
      if new_push_boxes.empty?
        can_move = true
      else
        push_boxes += new_push_boxes
      end
      check_positions = new_check_pos
    end
    if wall_found
      next
    else
      boxes -= push_boxes
      push_boxes.each do |box|
        boxes << [[box[0][0]+dir[0], box[0][1]+dir[1]],[box[1][0]+dir[0], box[1][1]+dir[1]]]
      end
      bot = new_pos
    end
  else
    # if no boxes or walls, let's just move
    bot = new_pos
  end
end

print_warehouse(boxes, walls, bot)

result = 0
boxes.each do |box|
  result += box[0][0] + box[0][1]*100
end

puts "Part1: #{result}"
