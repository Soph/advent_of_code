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

@distances = {}
valves.values.each do |valve|
  @distances[valve.name] = {valve.name => 0}
  loop do
    found = false
    known = @distances[valve.name].clone
    known.each do |name, distance|
      valves[name].next.each do |n|
        if @distances[valve.name][n].nil? || @distances[valve.name][n] > distance + 1
          @distances[valve.name][n] = distance + 1
          found = true
        end
      end
    end
    break if !found
  end
end

open_valves = Set.new
current = "AA"
@max_released = 0

time = 30
total_released = 0
current_rate = 0
openable_valves_count = valves.values.select{|valve| valve.rate > 0 }.size
variants = [{open: ["AA"], rate: 0, total_release: 0, time_left: 30}]
loop do
  new_variants = []
  found = false
  variants.each do |variant|
    valve = valves[variant[:open].last]
    valves.values.each do |v|
      next if v.rate == 0
      next if variant[:open].include?(v.name)
      found = true
      distance = @distances[valve.name][v.name]
      time_spent = distance + 1 # move + open
      new_variant = {
        open: variant[:open] + [v.name],
        total_release: variant[:total_release] + time_spent * variant[:rate],
        rate: variant[:rate] + v.rate,
        time_left: variant[:time_left] - time_spent
      }
      new_variants << new_variant
    end
  end
  break if !found
  variants = new_variants
end
# variants.map {|m| puts m.inspect}
final = variants.map do |v|
  v[:total_release] += v[:rate] * v[:time_left]
  v[:time_left] = 0
  v
end

puts "Result 1: #{final.sort{|x,y| y[:total_release]<=>x[:total_release]}.first.inspect}"

time = 26
total_released = 0
current_rate = 0
openable_valves_count = valves.values.select{|valve| valve.rate > 0 }.size
variants = [{open: ["AA"], elephant: ["AA"], me: ["AA"], rate_e: 0, total_release_e: 0, time_left_e: 26, rate_m: 0, total_release_m: 0, time_left_m: 26}]
loop do
  new_variants = []
  found = false
  variants.each do |variant|
    distances = {}
    distances[:elephant] = valves.values.select{|v| v.rate > 0 && !variant[:open].include?(v.name)}.map{|v| [v.name, @distances[variant[:elephant].last][v.name]] }.to_h
    distances[:me] = valves.values.select{|v| v.rate > 0 && !variant[:open].include?(v.name)}.map{|v| [v.name, @distances[variant[:me].last][v.name]] }.to_h
    #puts variant.inspect
    #puts distances.inspect

    [[:me, :elephant], [:elephant, :me]].each do |actors|
      distances[actors[0]].each do |name_1, distance_1|
        distances[actors[1]].each do |name_2, distance_2|
          next if name_1 == name_2
          next if distance_1 > distance_2
          found = true
          time_spent_1 = distance_1 + 1
          time_spent_2 = distance_2 + 1 - time_spent_1
          if distance_1 == distance_2
            time_spent_2 = 0
          end
          new_variant = {
            open: variant[:open] + [name_1, name_2],
            time_left: variant[:time_left] - (distance_2 + 1),
            rate: variant[:rate] + valves[name_1].rate + valves[name_2].rate,
            elephant: variant[:elephant].clone,
            me: variant[:me].clone,
            total_release: variant[:total_release]
          }
          new_variant[actors[0]] << name_1 
          new_variant[actors[1]] << name_2
          new_variant[:total_release] += (time_spent_1 * variant[:rate]) + (time_spent_2 * (variant[:rate] + valves[name_1].rate))
          new_variants << new_variant
          if new_variant[:open][0..3] == ["AA","DD", "JJ", "BB"]
            puts distances.inspect
            puts "#{variant.inspect} -> #{new_variant.inspect}"
          end
        end
      end
    end
  end
  break if !found
  variants = new_variants
end
# variants.map {|m| puts m.inspect}
final = variants.map do |v|
  v[:total_release] += v[:rate] * v[:time_left]
  v[:time_left] = 0
  v
end

puts "Result 2: #{final.sort{|x,y| y[:total_release]<=>x[:total_release]}.first.inspect}"

#"DD", "JJ", "BB", "HH", "CC", "EE"
# E "DD", "HH", "EE"
# M "JJ", "BB", "CC"

# {:elephant=>{"BB"=>2, "CC"=>1, "EE"=>1, "HH"=>4}, :me=>{"BB"=>3, "CC"=>4, "EE"=>4, "HH"=>7}}
# E -> HH -> 4
# M -> BB -> 3
# {:open=>["AA", "DD", "JJ"], :time_left=>23, :rate=>41, :elephant=>["AA", "DD"], :me=>["AA", "JJ"], :total_release=>20} -> 
# {:open=>["AA", "DD", "JJ", "BB", "HH"], :time_left=>18, :rate=>76, :elephant=>["AA", "DD", "HH"], :me=>["AA", "JJ", "BB"], :total_release=>238}