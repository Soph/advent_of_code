file = File.open(ARGV[0])
instructions = file.readlines.map(&:chomp).map(&:split)

def print_screen(screen)
  screen.each do |lines|
    puts lines.join
  end
end

def pos_to_xy(pos)
  return pos%40, pos/40
end

screen = []
6.times do |y|
  40.times do |x|
    screen[y] ||= []
    screen[y] << "."
  end
end

values = []
x = 1
current_op = nil
current_op_cycles_left = 0

(1..240).each do |cycle|
  if current_op_cycles_left == 0
    if !current_op.nil? && current_op[0] == "addx"
      x += current_op[1].to_i
    end

    current_op = instructions.shift
    if current_op[0] == "addx"
      current_op_cycles_left = 2
    else
      current_op_cycles_left = 1
    end
  end
  current_op_cycles_left -= 1

  draw_x, draw_y = pos_to_xy(cycle-1)
  sprite_m_x, sprite_m_y = pos_to_xy(x)
  if sprite_m_x - 1 <= draw_x && sprite_m_x + 1 >= draw_x
    screen[draw_y][draw_x] = "#"
  end
  print_screen(screen)

  if [20, 60, 100, 140, 180, 220].include?(cycle)
    values << x * cycle
  end
  break if cycle >= 240
end

puts "Result1: #{values.sum}"
print_screen(screen)



