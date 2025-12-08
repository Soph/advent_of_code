file = File.open(ARGV[0])
coordinates = file.readlines.map(&:chomp).map { |l| l.split(',').map(&:to_i) }

# puts coordinates.inspect

def distance(a, b)
  Math.sqrt((a[0] - b[0])**2 + (a[1] - b[1])**2 + (a[2] - b[2])**2)
end

distances = {}
coordinates.combination(2).each do |pair|
  distances[distance(pair[0], pair[1])] = pair
end

connections = Set.new
index = 0
sorted_keys = distances.keys.sort
loop do
  key = sorted_keys[index]
  index += 1
  pair = distances[key]
  existing = connections.select { |c| c.include?(pair[0]) || c.include?(pair[1]) }

  if existing.size == 2
    connections.subtract(existing)
    connections << existing.flatten(1).uniq.sort
  else
    existing = existing.first
    # skip if both are part of a connection already
    if !existing.nil? && existing.include?(pair[0]) && existing.include?(pair[1])
      # puts "Skipping #{pair}"
      next
    end

    if existing
      connections.subtract([existing])
      connections << (existing + pair).uniq.sort
    else
      connections << pair.sort
    end
  end

  puts "Part 1 (10): #{connections.map(&:size).sort.reverse.take(3).inject(:*)}" if index == 10
  puts "Part 1 (1000): #{connections.map(&:size).sort.reverse.take(3).inject(:*)}" if index == 1000
  next unless index > 10 && connections.size == 1 && connections.first.size == coordinates.size

  puts "Part 2: #{pair[0][0] * pair[1][0]}"
  break
end
