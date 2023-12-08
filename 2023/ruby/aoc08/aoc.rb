file = File.open(ARGV[0])
sections = file.read.split("\n\n")

directions = sections[0].chars
network = {}
starts = []
sections[1].split("\n").each do |node|
  network[node.chars[0..2].join] = [node.chars[7..9].join, node.chars[12..14].join]
  starts << node.chars[0..2].join if node.chars[2] == 'A'
end

count = 0
pos = 0
current = "AAA"
while true do
  break if network[current].nil?
  dir = directions[pos] == 'R' ? 1 : 0
  current = network[current][dir]
  pos = (pos + 1) % directions.size
  count += 1
  break if current == "ZZZ"
end

puts "Part 1: #{count}"

paths = {}
starts.each do |start|
  count = 0
  pos = 0
  current = start
  while true do
    break if network[current].nil?
    dir = directions[pos] == 'R' ? 1 : 0
    current = network[current][dir]
    pos = (pos + 1) % directions.size
    count += 1
    break if current.chars[2] == "Z"
  end
  paths[start] = count
end
puts "Part 2: #{paths.values.reduce(1, :lcm)}"
