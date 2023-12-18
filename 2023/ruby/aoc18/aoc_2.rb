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

#ranges = []
CORNERS = [[0,0]]

instructions.each do |instruction|
  parts = instruction.split(" ")
  #string = parts[2].gsub(/[\(\)\#]/,'')
  #times = string[0...5].to_i(16)
  #direction = string[5..]
  direction = movements[parts[0]]
  times = parts[1].to_i
  case direction
  when "0"
    #ranges << [(pos[0]..pos[0]+times),pos[1]]
    pos = [pos[0]+times, pos[1]]
  when "2"
    #ranges << [(pos[0]-times..pos[0]),pos[1]]
    pos = [pos[0]-times, pos[1]]
  when "1"
    #ranges << [pos[0],(pos[1]..pos[1]+times)]
    pos = [pos[0], pos[1]+times]
  when "3"
    #ranges << [pos[0],(pos[1]-times..pos[1])]
    pos = [pos[0], pos[1]-times]
  end
  CORNERS << pos unless CORNERS.include?(pos)
end

puts CORNERS.inspect


#puts HOLES.inspect
sorted_x = CORNERS.sort_by{|corner| corner[0]}
MIN_X = sorted_x.first[0] - 1
MAX_X = sorted_x.last[0] + 1
sorted_y = CORNERS.sort_by{|corner| corner[1]}
MIN_Y = sorted_y.first[1] - 1
MAX_Y = sorted_y.last[1] + 1

rows_with_corners = CORNERS.map{|corner| corner[1]}.uniq.sort
puts rows_with_corners.inspect
opened = []
active_columns = Set.new
sum = 0
last_y = nil
rows_with_corners.each_with_index do |y, i|
  sorted = CORNERS.select {|c| c[1] == y}.sort_by{|c| c[0]}
  sorted.map{|c| c[0] }.each do |x|
    if active_columns.include?(x)
      active_columns.delete(x) # corner is closing
    else
      active_columns.add(x)
    end
  end
  next_y = rows_with_corners[i+1]
  if next_y
    puts "#{active_columns.to_a.sort} - #{next_y} - #{y}: #{((next_y - y).abs)}"
    active_columns.to_a.sort.each_slice(2) do |x1, x2|
      puts "#{((x2 - x1) + 1)} * #{(((next_y + 1) - y).abs)}"
      puts "#{((x2 - x1) + 1) * (((next_y + 1) - y).abs)}"
      sum += ((x2 - x1) + 1) * (((next_y + 1) - y).abs)
    end
  end
  #exit if i > 10
end

puts sum