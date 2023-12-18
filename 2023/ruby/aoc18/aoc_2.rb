require "set"
file = File.open(ARGV[0])
instructions = file.readlines.map(&:chomp)

HOLES = {}
HOLES[[0,0]] = ''

movements = {
  "R" => "0",
  "U" => "3",
  "L" => "2",
  "D" => "1",
}

pos = [0,0]

CORNERS = [[0,0]]

instructions.each do |instruction|
  parts = instruction.split(" ")
  string = parts[2].gsub(/[\(\)\#]/,'')
  times = string[0...5].to_i(16)
  direction = string[5..]
  #direction = movements[parts[0]]
  #times = parts[1].to_i
  case direction
  when "0"
    pos = [pos[0]+times, pos[1]]
  when "2"
    pos = [pos[0]-times, pos[1]]
  when "1"
    pos = [pos[0], pos[1]+times]
  when "3"
    pos = [pos[0], pos[1]-times]
  end
  CORNERS << pos unless CORNERS.include?(pos)
end

#puts HOLES.inspect
sorted_x = CORNERS.sort_by{|corner| corner[0]}
MIN_X = sorted_x.first[0] - 1
MAX_X = sorted_x.last[0] + 1
sorted_y = CORNERS.sort_by{|corner| corner[1]}
MIN_Y = sorted_y.first[1] - 1
MAX_Y = sorted_y.last[1] + 1

# (MIN_Y...MAX_Y).each do |y|
#   (MIN_X...MAX_X).each do |x|
#     if CORNERS.include?([x,y])
#       putc "#"
#     else
#       putc "."
#     end
#   end
#   puts
# end

rows_with_corners = CORNERS.map{|corner| corner[1]}.uniq.sort

opened = []
active_columns = []
sum = 0
last_sum = 0
rows_with_corners.each_with_index do |y, i|
  deleted = Set.new
  added = Set.new
  sorted = CORNERS.select {|c| c[1] == y}.sort_by{|c| c[0]}
  sorted.map{|c| c[0] }.each do |x|
    if active_columns.include?(x)
      deleted << x
    else
      active_columns << x
      added << x
    end
  end
  active_columns.sort!
  inside = false
  #puts active_columns.inspect
  n = 0
  was_double_corner = false
  while n < active_columns.size
    toggle = true
    x1 = active_columns[n]
    x2 = active_columns[n+1]
    if x2.nil?
      #puts "EOL Add: 1"
      sum += 1
      break
    end
    #puts "Check: #{x1} - #{x2}"
    
    if deleted.include?(x1) && deleted.include?(x2) && !was_double_corner
      # LJ no impact on inside
      sum += (x2 - x1)
      #puts "LJ Add: #{(x2 - x1)}"
      was_double_corner = true
    elsif deleted.include?(x1) && added.include?(x2) && !was_double_corner
      # L7
      #puts "L7 Add: #{(x2 - x1)}"
      sum += (x2 - x1)
      toggle = false
      was_double_corner = true
    elsif added.include?(x1) && deleted.include?(x2) && !was_double_corner
      # FJ
      sum += (x2 - x1)
      #puts "FJ Add: #{(x2 - x1)}"
      toggle = false
      was_double_corner = true
    elsif added.include?(x1) && added.include?(x2) && !was_double_corner
      # F7
      #puts "F7 Add: #{(x2 - x1)}"
      sum += (x2 - x1)
      was_double_corner = true
    else
      was_double_corner = false if was_double_corner
      inside = !inside
      toggle = false
      if inside
        sum += (x2 - x1)
        #puts "| Add: #{(x2 - x1)}"
      else
        #puts "| Add: 1 (outside)"
        sum += 1 # corner counts too
      end
    end
    if toggle
      inside = !inside
    end

    n += 1
  end
  #puts "Sum (after corners): #{sum}"
  deleted.each{ |corner| active_columns.delete(corner) }
  next_y = rows_with_corners[i+1] || 0
  #puts "NextY: #{next_y}"
  #puts "#{y}: #{sum - last_sum} - #{sum}"
  lines = (next_y - y).abs - 1
  #puts "#{active_columns} - #{next_y} - #{y}: #{((next_y - y).abs)}"
  active_columns.each_slice(2) do |x1, x2|
    #puts "#{((x2 - x1) + 1)} * #{lines}"
    #puts "#{((x2 - x1) + 1) * lines}"
    sum += ((x2 - x1) + 1) * lines
  end
  #puts "Sum (after all): #{sum}"
  #puts 
  #puts
  last_sum = sum
  #break if i > 30
end

puts sum