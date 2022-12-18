require "set"
file = File.open(ARGV[0])

Point = Struct.new(:x, :y, :z) do
  def to_s
    "(#{x},#{y},#{z})"
  end
end

Cube = Struct.new(:x, :y, :z) do
  def to_s
    "(#{x},#{y},#{z})"
  end

  def sides
    [
      Set[Point.new(x+0,y+0,z+0),Point.new(x+1,y+0,z+0),Point.new(x+0,y+1,z+0),Point.new(x+1,y+1,z+0)], #front
      Set[Point.new(x+0,y+0,z+0),Point.new(x+0,y+1,z+0),Point.new(x+0,y+0,z+1),Point.new(x+0,y+1,z+1)], #left
      Set[Point.new(x+0,y+0,z+0),Point.new(x+0,y+0,z+1),Point.new(x+1,y+0,z+0),Point.new(x+1,y+0,z+1)], #bottom
      Set[Point.new(x+1,y+1,z+1),Point.new(x+1,y+0,z+0),Point.new(x+1,y+1,z+0),Point.new(x+1,y+0,z+1)], #right
      Set[Point.new(x+1,y+1,z+1),Point.new(x+0,y+1,z+0),Point.new(x+1,y+1,z+0),Point.new(x+0,y+1,z+1)], #top
      Set[Point.new(x+1,y+1,z+1),Point.new(x+0,y+0,z+1),Point.new(x+1,y+0,z+1),Point.new(x+0,y+1,z+1)], #rear
    ]
  end
end

max_x = 0
max_y = 0
max_z = 0
cubes = file.readlines.map(&:chomp).map do |l|
  cube = Cube.new(*l.split(",").map{|number| number.to_i + 1}) # shifting every by one in all 3 axis so searching from 0 works for part 2
  max_x = cube.x if cube.x > max_x
  max_y = cube.y if cube.y > max_y
  max_z = cube.z if cube.z > max_z
  cube
end

sides = cubes.map(&:sides).flatten.tally.select{|cube, count| count == 1}.keys
puts "Result1: #{sides.uniq.size}"

# need to check the outsides on the max edges too, so add + 1 to search there too
max_x += 1
max_y += 1
max_z += 1

searchs = [Cube.new(0,0,0)]
visited = Set.new
outsides = Set.new
max = [0,0,0]
loop do
  search = searchs.shift
  break if search.nil?
  next if visited.include?(search)
  visited << search
  outsides += sides & search.sides
  next_cubes = []

  (search.sides - sides).each do |side|
    case search.sides.index(side)
    when 0 #front
      puts "bounds" && next if search.z - 1 < 0
      next_cubes << Cube.new(search.x, search.y, search.z - 1)
    when 1 #left
      puts "bounds" && next if search.x - 1 < 0
      next_cubes << Cube.new(search.x - 1, search.y, search.z)
    when 2 #bottom
      puts "bounds" && next if search.y - 1 < 0
      next_cubes << Cube.new(search.x, search.y - 1, search.z)
    when 3 #right
      puts "bounds" && next if search.x + 1 > max_x
      next_cubes << Cube.new(search.x + 1, search.y, search.z)
    when 4 #top
      puts "bounds" && next if search.y + 1 > max_y
      next_cubes << Cube.new(search.x, search.y + 1, search.z)
    when 5 #rear
      puts "bounds" && next if search.z + 1 > max_z
      next_cubes << Cube.new(search.x, search.y, search.z + 1)
    end
  end
  searchs += next_cubes
  searchs.uniq!
  break if searchs.empty?
  if search.x > max[0]
    max[0] = search.x
    puts "#{max.inspect} - #{outsides.size} - #{searchs.size}"
  elsif search.y > max[1]
    max[1] = search.y
    puts "#{max.inspect} - #{outsides.size} - #{searchs.size}"
  elsif search.z > max[2]
    max[2] = search.z
    puts "#{max.inspect} - #{outsides.size} - #{searchs.size}"
  end
end

puts "Result2: #{outsides.size}"

