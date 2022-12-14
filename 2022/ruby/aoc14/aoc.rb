require "set"
file = File.open(ARGV[0])
scans = file.readlines.map(&:chomp)

Point = Struct.new(:x, :y) do
  def to_s
    "(#{x},#{y})"
  end
end

def visualize(rocks, positions, deepest)
  min_y = positions.map{|p| p.y}.min
  max_y = positions.map{|p| p.y}.max
  min_x = positions.map{|p| p.x}.min
  max_x = positions.map{|p| p.x}.max

  (min_y-1..max_y+1).each do |y|
    row = (min_x-1..max_x+1).map do |x|
      pos = Point.new(x,y)
      if rocks.include?(pos)
        "#"
      elsif positions.include?(pos)
        "o"
      else
        "."
      end
    end
    puts row.join
  end
end

positions = Set.new
scans.each do |scan|
  last = nil
  scan.split(" -> ").each do |position|
    if last.nil?
      last = position.split(",").map(&:to_i)
    else
      target = position.split(",").map(&:to_i)
      if target[0] == last[0]
        ([last[1], target[1]].min..[last[1], target[1]].max).each do |y|
          positions << Point.new(target[0], y)
        end
      else
        ([last[0], target[0]].min..[last[0], target[0]].max).each do |x|
          positions << Point.new(x, target[1])
        end
      end
      last = target
    end
  end
end

rocks = positions.clone
directions = [[0,1], [-1,1], [1,1]]
#deepest = positions.sort{|p| p.y}.last.y
deepest = positions.map{|p| p.y}.max

count = 0
done = false
out_of_bounds = nil
while !done do
  count += 1
  last = Point.new(500,0)
  while true do
    options = directions.map do |dir|
      target = Point.new(last.x + dir[0], last.y + dir[1])
      if positions.include?(target)
        nil
      else
        target
      end
    end.compact
    if options.size == 0 && !(last.x == 500 && last.y == 0)
      # come to rest
      positions << last
      break
    elsif options[0].y > deepest
      out_of_bounds = options[0]
      done = true
      break
    end
    last = options[0]
  end
  #puts positions.inspect
end

visualize(rocks, positions, deepest)
puts "Result1: #{count-1}"

positions = rocks.clone
count = 0
done = false
out_of_bounds = nil
while !done do
  count += 1
  last = Point.new(500,0)
  while true do
    options = directions.map do |dir|
      target = Point.new(last.x + dir[0], last.y + dir[1])
      if positions.include?(target)
        nil
      elsif target.y == deepest + 2
        nil
      else
        target
      end
    end.compact
    if options.size == 0 && !(last.x == 500 && last.y == 0)
      positions << last
      break
    elsif options.size == 0 && last.x == 500 && last.y == 0
      done = true
      break
    end
    last = options[0]
  end
  #puts positions.inspect
end

visualize(rocks, positions, deepest)
puts "Result2: #{count}"