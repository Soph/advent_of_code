require "set"

file = File.open(ARGV[0])
data = file.readlines.map(&:chomp).join("\n").split("\n\n")

wires = {}
data[0].split("\n").map do |line|
  info = line.split(": ")
  wires[info[0]] = info[1] == "1"
end

mapping = {
  "AND" => "&&",
  "OR" => "||",
  "XOR" => "^"
}

gates = data[1].split("\n").map do |line|
  match = line.match(/(.*) (AND|OR|XOR) (.*) \-\> (.*)/)
  res = match[1..4]
  res[1] = mapping[res[1]]
  res
end

puts wires.inspect

missing = true
while missing
  missing = false
  gates.each do |gate|
    if wires[gate[0]].nil? || wires[gate[2]].nil?
      missing = true
      next
    end
    wires[gate[3]] = eval("#{wires[gate[0]]} #{gate[1]} #{wires[gate[2]]}")
  end
end

wires.keys.sort.each{|key| puts "#{key}: #{wires[key]? 1 : 0}"}
puts wires.keys.select{|wire| wire.start_with?("z")}.sort.reverse.map{|key| wires[key] ? 1 : 0}.join.to_i(2)

