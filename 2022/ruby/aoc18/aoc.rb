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
loop do
  search = searchs.shift
  visited << search
  outs = sides & search.sides
  if !outs.empty?
    puts "outs: #{outs.map(&:to_s).join(" - ")}"
    outsides += outs
  end
  next_cubes = Set.new
  (search.sides - sides).each do |side|
    case search.sides.index(side)
    when 0 #front
      next if search.z - 1 < 0
      next_cubes << Cube.new(search.x, search.y, search.z - 1)
    when 1 #left
      next if search.x - 1 < 0
      next_cubes << Cube.new(search.x - 1, search.y, search.z)
    when 2 #bottom
      next if search.y - 1 < 0
      next_cubes << Cube.new(search.x, search.y - 1, search.z)
    when 3 #right
      next if search.x + 1 > max_x
      next_cubes << Cube.new(search.x + 1, search.y, search.z)
    when 4 #top
      next if search.y + 1 > max_y
      next_cubes << Cube.new(search.x, search.y + 1, search.z)
    when 5 #rear
      next if search.z + 1 > max_z
      next_cubes << Cube.new(search.x, search.y, search.z + 1)
    end
  end
  next_cubes = next_cubes.select{|c| !visited.include?(c)}.select{|c| !cubes.include?(c)}
  break if next_cubes.empty?
  searchs += next_cubes
  searchs.uniq!
  # puts "Search: #{searchs.map(&:to_s).join(" - ")}"
  # puts "Visited: #{visited.map(&:to_s).join(" - ")}"
  puts visited.count
  puts searchs.count
  #sleep 2
end

puts "Result2: #{outsides.size}"

