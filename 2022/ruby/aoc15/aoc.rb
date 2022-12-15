require "set"
file = File.open(ARGV[0])
search_y = ARGV[1].to_i
scans = file.readlines.map(&:chomp)


Point = Struct.new(:x, :y) do
  def to_s
    "(#{x},#{y})"
  end

  def distance(point)
    (self.x - point.x).abs + (self.y - point.y).abs
  end
end

sensors = {}
beacons = Set.new
scans.each do |scan|
  data = scan.match(/Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)/)
  sensor = Point.new(data[1].to_i, data[2].to_i)
  beacon = Point.new(data[3].to_i, data[4].to_i)

  beacons << beacon
  sensors[sensor] = sensor.distance(beacon)
end

current_x = 0
not_possible = Set.new
last_distances = {}
point = Point.new(current_x, search_y)
sensors.each do |sensor, value|
  last_distances[sensor] = sensor.distance(point)
end

skip_sensors = Set.new
loop do
  break if skip_sensors.size == sensors.size
  sensors.each do |sensor, max_distance|
    next if skip_sensors.include?(sensor)
    found = false
    point_right = Point.new(current_x, search_y)
    distance_right = sensor.distance(point_right)
    if sensor.distance(point_right) <= max_distance
      found = true
      not_possible << point_right
    end
    point_left = Point.new(-current_x, search_y)
    distance_left = sensor.distance(point_left)
    if sensor.distance(point_left) <= max_distance
      found = true
      not_possible << point_left
    end
    new_last_distance = [distance_left, distance_right].min
    if !found && last_distances[sensor] < new_last_distance
      # moving away from sensor in both directions, no need to check anymore
      skip_sensors << sensor
      puts "Skip: #{sensor} Left: #{sensors.size - skip_sensors.size}"
    end
  end
  current_x += 1
  #puts current_x
end

puts (not_possible - beacons).size

skip_sensors = Set.new
last_distances = {}
point = Point.new(current_x, search_y)
sensors.each do |sensor, value|
  last_distances[sensor] = sensor.distance(point)
end

min_x = 0
min_y = 0
max_x = search_y * 2
max_y = search_y * 2

(min_y..max_y).each do |y|
  x = 0
  loop do
    break if x >= max_x
    point = Point.new(x, y)

    found = false
    sensors.each do |sensor, max_distance|
      distance = sensor.distance(point)
      if distance <= max_distance
        if point.x <= sensor.x # sensor is right
          x += 2 * (sensor.x - point.x)
          found = true
          break
        elsif point.x > sensor.x # sensor is left
          x += max_distance - distance
          found = true
          break
        end
      end
    end

    if found == false
      puts point
      puts "Result2: #{point.x*4000000+point.y}"
      exit
    end
    x+=1
  end
  puts y if y % 100000 == 0
end
        
# ........lxxxx........
# .......xxxxxxx.......
# ......xxxxSxxxx......