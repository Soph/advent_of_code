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

dimensions_x = []
dimensions_y = []
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
    dimensions_y[x] ||= [lines.size, 0]
    dimensions_y[x][0] = y if y < dimensions_y[x][0]
    dimensions_y[x][1] = y if y > dimensions_y[x][1]  
  end
  dimensions_x << [min_x, max_x]
end

cur = Point.new(dimensions_x[0][0], 0)
directions = [[1,0], [0,1], [-1,0], [0,-1]]
direction_index = 0
movements.each do |move|
  puts "#{cur}: #{move}"
  move.steps.times do
    new_pos = cur.move(*directions[direction_index])
    if direction_index % 2 == 0 # horizontal
      if new_pos.x > dimensions_x[new_pos.y][1] # move out right, wrap left
        puts "#{cur}: move out right, wrap left"
        new_pos = Point.new(dimensions_x[new_pos.y][0], new_pos.y)
      elsif new_pos.x < dimensions_x[new_pos.y][0] # move out left, wrap right
        puts "#{cur}: move out left, wrap right"
        new_pos = Point.new(dimensions_x[new_pos.y][1], new_pos.y)
      end
    else
      if new_pos.y > dimensions_y[new_pos.x][1] # move out 
        puts "#{cur}: move out bottom, wrap top"
        new_pos = Point.new(new_pos.x, dimensions_y[new_pos.x][0])
      elsif new_pos.y < dimensions_y[new_pos.x][0]
        puts "#{cur}: move out top, wrap bottom"
        new_pos = Point.new(new_pos.x, dimensions_y[new_pos.x][1])
      end
    end
    if walls.include?(new_pos)
      puts "#{cur}: Hit a wall at #{new_pos}"
      break
    end
    puts "Move: #{cur} -> #{new_pos}"
    cur = new_pos
  end
  if move.turn == "L"
    puts "#{cur}: turn left (#{direction_index})"
    if direction_index == 0
      direction_index = (directions.size - 1)
    else
      direction_index = (direction_index - 1) % (directions.size)
    end
  elsif move.turn == "R"
    puts "#{cur}: turn right (#{direction_index})"
    direction_index = (direction_index + 1) % (directions.size)
  end
  puts "#{direction_index}"
  #sleep 5
end

puts cur.to_s
result = 1000 * (cur.y + 1) + 4 * (cur.x + 1) + direction_index
puts "Result: #{result}"