require "set"
file = File.open(ARGV[0])

Valve = Struct.new(:name, :rate, :next) do
  def to_s
    "(#{name}: #{rate})"
  end

  def self.parse(raw)
    data = raw.match(/Valve (.*) has flow rate=(\d+); tunnels? leads? to valves? (.*)/)
    next_valves = data[3].split(", ")
    Valve.new(data[1], data[2].to_i, next_valves)
  end
end

valves = file.readlines.map(&:chomp).map{|line| valve = Valve.parse(line); [valve.name,valve]}.to_h
puts valves.inspect
open_valves = Set.new
start = "AA"

result = {}
@max_released = 0
#@paths = {}

def step(position, last_position, valves, open_valves, time, pressure_released)
  if pressure_released > @max_released
    puts "#{time}: #{position} #{pressure_released} #{open_valves.map(&:name).sort} #{open_valves.map(&:name)} next: #{valves[position].next}"
    puts "new max: #{pressure_released}"
    @max_released = pressure_released
  end
  if time == 0
    #puts "time is up: #{position} #{pressure_released}"
    return pressure_released
  end

  if open_valves.size == valves.select{|o,v| v.rate > 0}.size
    return pressure_released + (open_valves.map(&:rate).sum * time)
  end

  valve = valves[position]
  total_released = pressure_released + open_valves.map(&:rate).sum
  variants = []

  
  # opening
  if !open_valves.include?(valve) && valve.rate > 0
    variants << step(position, nil, valves, open_valves + [valve], time - 1, total_released)
  end
  
  # moving
  valve.next.each do |next_valve|
    next if next_valve == last_position

    variants << step(next_valve, position, valves, open_valves, time - 1, total_released)
  end

  if variants.empty?
    return pressure_released + (open_valves.map(&:rate).sum * time)
  end

  result = variants.max
  #puts "#{time}: #{position} #{pressure_released} -> #{result}"
  #sleep 1
  result
end

variants = step(start, nil, valves, open_valves, 30, 0)
puts variants.inspect