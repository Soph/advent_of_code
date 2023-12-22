require "Set"

file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

cubes = lines.map{|line| line.split("~").map{|part| part.split(",").map(&:to_i)}}

puts cubes.inspect

cubes = cubes.sort_by{|cube| cube[0][2] }

dx = [0,0,1]

# falling
occupied = {}
fallen_cubes = []
cubes.each_with_index do |cube, i|
  z = cube[0][2]
  while true
    z -= 1
    break if z <= 0
    can_fall = true
    (cube[0][1]..cube[1][1]).each do |y|
      (cube[0][0]..cube[1][0]).each do |x|
        can_fall = false if occupied[[x,y,z]]
      end
    end
    break unless can_fall
    cube[0][2] -= 1
    cube[1][2] -= 1
  end
  (cube[0][2]..cube[1][2]).each do |z|
    (cube[0][1]..cube[1][1]).each do |y|
      (cube[0][0]..cube[1][0]).each do |x|
        occupied[[x,y,z]] = i
      end
    end
  end
end

# brick supports [bricks]
supported = {}
puts occupied.inspect

cubes.each_with_index do |cube, i|
  (cube[0][1]..cube[1][1]).each do |y|
    (cube[0][0]..cube[1][0]).each do |x|
      above_z = cube[1][2]+1
      if occupied[[x,y,above_z]]
        supported[occupied[[x,y,above_z]]] ||= Set.new
        supported[occupied[[x,y,above_z]]] << i
      end
    end
  end
end

puts
puts supported.inspect

need_to_stay = Set.new
cubes.each_index do |i|
  if !supported[i].nil? && supported[i].size == 1
    need_to_stay << supported[i]
  end
end
puts need_to_stay.inspect
puts cubes.size - need_to_stay.size