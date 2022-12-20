require "set"
file = File.open(ARGV[0])
moves = file.readlines.map(&:chomp)[0].chars

Point = Struct.new(:x, :y) do
  def to_s
    "(#{x},#{y})"
  end
end

Rock = Struct.new(:points) do
  def to_s
    points.map(&:to_s).join(" ")
  end

  def move_direction(dir, cave)
    if dir == '>'
      new_points = points.map {|p| Point.new(p.x+1, p.y)}
    elsif dir == '<'
      new_points = points.map {|p| Point.new(p.x-1, p.y)}
    elsif dir == '|'
      new_points = points.map {|p| Point.new(p.x, p.y-1)}
    end
    return false if new_points.any? {|p| p.x < cave.min_x}
    return false if new_points.any? {|p| p.x > cave.max_x}
    return false if new_points.any? {|p| p.y < cave.min_y}
    return false if cave.collision?(new_points)

    self.points = new_points

    return true
  end

  def copy_to_start_position(x:, y:)
    new_points = points.map {|p| Point.new(p.x + x, p.y + y)}
    #puts "#{x}, #{y}: #{points.map(&:to_s)}"
    return Rock.new(new_points)
  end
end

Cave = Struct.new(:min_x, :max_x, :stones) do
  def start_y
    return 3 if stones.empty?
    max_y + 4
  end

  def max_y
    stones.map(&:y).max
  end

  def min_y
    0
  end

  def start_x
    min_x + 2
  end

  def reduce!
    #puts "Before: #{stones.inspect}"
    min_y = (0..max_x).map{|x| stones.select {|p| p.x == x}.map(&:y).max || 0}.compact.min
    self.stones = stones.select {|p| p.y >= min_y}.map{|p| Point.new(p.x, p.y-min_y)}
    #puts (0..max_x).map{|x| stones.select {|p| p.x == x}.map(&:y).max || 0}.inspect
    #puts "After: #{stones.inspect}"
    #puts
    return min_y
  end

  def cache_key
    #puts stones.map(&:to_s).join("-")
    stones.map(&:to_s).join("-")
  end

  def collision?(points)
    !(stones & points).empty?
  end

  def visualize(points)
    (0..start_y+3).to_a.reverse.each do |y|
      line = ""
      (0..max_x).to_a.each do |x|
        if stones.include?(Point.new(x,y))
          line << "#"
        elsif points.include?(Point.new(x,y))
          line << "@"
        else
          line << "."
        end
      end
      puts line
    end
    puts "-------"
  end
end

rocks = [
    Rock.new([Point.new(0,0), Point.new(1,0), Point.new(2,0), Point.new(3,0)]),
    Rock.new([Point.new(1,0), Point.new(0,1), Point.new(1,1), Point.new(2,1), Point.new(1,2)]),
    Rock.new([Point.new(0,0), Point.new(1,0), Point.new(2,0), Point.new(2,1), Point.new(2,2)]),
    Rock.new([Point.new(0,0), Point.new(0,1), Point.new(0,2), Point.new(0,3)]),
    Rock.new([Point.new(0,0), Point.new(0,1), Point.new(1,0), Point.new(1,1)])
]

current_rock = 0
move_n = 0
cave = Cave.new(0, 6, Set.new)
truncates = 0
cache = {}
rock_i = 0
total_y = 0
counter = ARGV[1].to_i
i = 0
shifted = false
while i < counter
  cache_key = [rock_i, move_n]
  cave_cache_key = cave.cache_key
  if !shifted && cache[cache_key] && cache[cache_key][cave_cache_key]
    cave.visualize([])
    shifted = true
    puts "Shifted from #{i} and #{total_y} #{move_n}"
    puts cache[cache_key][cave_cache_key]
    offset_count = i - cache[cache_key][cave_cache_key][:i]
    offset_y = total_y - cache[cache_key][cave_cache_key][:y]
    puts "Offsets #{offset_count} #{offset_y}"
    times = (counter - i) / offset_count
    puts "(#{counter} - #{i}) / #{offset_count} = #{times}"
    move_n += cache[cache_key][cave_cache_key][:moves]
    move_n += (cache[cache_key][cave_cache_key][:moves] * times)
    move_n %= moves.size
    total_y += offset_y * times
    i += offset_count * times
    rock_i += (offset_count * times)
    rock_i %= rocks.count
    puts "to #{i} and #{total_y} #{move_n}"
    cave.stones = cache[cache_key][cave_cache_key][:points]
    cave.visualize([])
  end
  rock = rocks[rock_i].copy_to_start_position(x: cave.start_x, y: cave.start_y)
  move_loop = 0
  loop do
    #cave.visualize(rock.points) if i == 48
    move = moves[move_n]
    puts "#{i} #{move}: #{rock.to_s}"
    rock.move_direction(move, cave)
    #cave.visualize(rock.points) if i == 48
    move_n = (move_n + 1) % moves.size
    move_loop += 1
    break unless rock.move_direction("|", cave)
    #puts "-> #{rock.to_s}"    
    #cave.visualize(rock.points)
  end
  cave.stones += rock.points
  total_y += cave.reduce!
  cache[cache_key] ||= {}
  cache[cache_key][cave_cache_key] = {
    moves: move_loop,
    points: cave.stones,
    y: total_y,
    i: i
  }  
  total_y += cave.reduce!
  rock_i = (rock_i + 1) % rocks.count
  i += 1

  #cave.visualize(rock.points)
  #puts "#{i}: #{cave.max_y}"
  #puts cave.stones.size
  puts i if i % 100000 == 0
  #sleep 1
  #exit if i == 4
end

puts cave.max_y + total_y