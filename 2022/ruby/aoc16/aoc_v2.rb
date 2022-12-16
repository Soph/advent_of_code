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

puts "Done Initializing"

open_valves = Set.new
current = "AA"
@max_released = 0
valves_with_rate = valves.values.select{|v| v.rate > 0 }

def run(valves, time: 30)
  variants = [{open: ["AA"], rate: 0, total_release: 0, time_left: time}]
  done = []
  loop do
    new_variants = []
    found = false
    variants.each do |variant|
      valves.each do |v|
        next if v.rate == 0
        next if variant[:open].include?(v.name)
        distance = @distances[variant[:open].last][v.name]
        time_spent = distance + 1 # move + open
        if variant[:time_left] <= time_spent
          done << variant.clone
          next
        end
        found = true
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
  final = (variants + done).map do |v|
    v[:total_release] += v[:rate] * v[:time_left]
    v[:time_left] = 0
    v
  end
  final.sort{|x,y| y[:total_release]<=>x[:total_release]}.first[:total_release]
end

puts "Result 1: #{run(valves_with_rate, time: 30)}"

max = 0
valves_with_rate.combination(valves_with_rate.length/2).each do |mine|
  v = run(mine, time: 26) + run(valves_with_rate-mine, time: 26)
  if v > max
    max = v
    puts max
  end
end

puts "Result 2: #{max}"

# openable_valves_count = valves.values.select{|valve| valve.rate > 0 }.size
# variants = [{
#   open: ["AA"], 
#   elephant: {
#     open: ["AA"],
#     rate: 0,
#     total_release: 0,
#     time_left: 26,
#   },
#   me: {
#     open: ["AA"],
#     rate: 0,
#     total_release: 0,
#     time_left: 26,
#   }}]
# done = []
# valves_with_rate = valves.values.select{|v| v.rate > 0 }
# open_map = {}
# loop do
#   new_variants = []
#   found = false
#   duplicates = Set.new
#   variants.each do |variant|
#     actor = variant[:elephant][:open].size > variant[:me][:open].size ? :me : :elephant
#     distances = valves_with_rate.select{|v| !variant[:open].include?(v.name) }.map{|v| [v.name, @distances[variant[actor][:open].last][v.name]] }.to_h
#     distances.each do |name, distance|
#       time_spent = distance + 1 # move + open
#       if variant[actor][:time_left] <= time_spent
#         done << variant.clone
#         next
#       end
#       new_variant = {}
#       new_variant[:open] = variant[:open] + [name]
#       new_variant[:elephant] = variant[:elephant].clone
#       new_variant[:me] = variant[:me].clone
#       new_variant[actor] = {
#         open: variant[actor][:open] + [name],
#         total_release: variant[actor][:total_release] + time_spent * variant[actor][:rate],
#         rate: variant[actor][:rate] + valves[name].rate,
#         time_left: variant[actor][:time_left] - time_spent
#       }
#       open_map[new_variant[actor][:open].sort] ||= {}
#       open_map[new_variant[actor][:open].sort][new_variant[actor][:open].last] ||= 0
#       projection = new_variant[actor][:rate] * new_variant[actor][:time_left] + new_variant[actor][:total_release]
#       #puts "#{new_variant[actor][:open].sort}: #{open_map[new_variant[actor][:open].sort].inspect} - #{projection}"
#       if open_map[new_variant[actor][:open].sort][new_variant[actor][:open].last] > projection
#         #puts "skip"
#         next
#       end
#       open_map[new_variant[actor][:open].sort][new_variant[actor][:open].last] = projection
#       lookup = [new_variant[:elephant][:open], new_variant[:me][:open]]
#       next if duplicates.include?(lookup) || duplicates.include?(lookup.reverse)
#       duplicates << lookup
#       found = true
#       new_variants << new_variant
#     end
#   end
#   break if !found
#   variants = new_variants
#   puts "#{variants.size} #{done.size}"
#   #variants.map {|m| puts m.inspect}
#   #sleep 1
# end
# # {:open=>["AA", "BB", "DD", "JJ"], :total_release=>171, :rate=>54, :time_left=>17}
# # ["AA", "DD", "BB", "JJ"]
# # {:open=>["AA", "DD", "BB", "JJ"], :total_release=>192, :rate=>54, :time_left=>17}
# # ["AA", "BB", "DD", "JJ"]

# # {:open=>["AA", "BB", "HH", "JJ"], :total_release=>371, :rate=>56, :time_left=>9}
# # ["AA", "BB", "HH", "JJ"]
# # ["AA", "HH", "BB", "JJ"]
# # {:open=>["AA", "HH", "BB", "JJ"], :total_release=>294, :rate=>56, :time_left=>9}

# # variants.map {|m| puts m.inspect}
# final = (variants + done).map do |v|
#   [:me, :elephant].each do |actor|
#     v[actor][:total_release] += v[actor][:rate] * v[actor][:time_left]
#     v[actor][:time_left] = 0
#   end
#   v[:total_release] = v[:elephant][:total_release] + v[:me][:total_release]
#   v
# end

# #final.map {|m| puts m.inspect}
# puts "Result 2: #{final.sort{|x,y| y[:total_release]<=>x[:total_release]}.first.inspect}"

# #"DD", "JJ", "BB", "HH", "CC", "EE"
# # E "DD", "HH", "EE"
# # M "JJ", "BB", "CC"

# # {:elephant=>{"BB"=>2, "CC"=>1, "EE"=>1, "HH"=>4}, :me=>{"BB"=>3, "CC"=>4, "EE"=>4, "HH"=>7}}
# # E -> HH -> 4
# # M -> BB -> 3
# # {:open=>["AA", "DD", "JJ"], :time_left=>23, :rate=>41, :elephant=>["AA", "DD"], :me=>["AA", "JJ"], :total_release=>20} -> 
# # {:open=>["AA", "DD", "JJ", "BB", "HH"], :time_left=>18, :rate=>76, :elephant=>["AA", "DD", "HH"], :me=>["AA", "JJ", "BB"], :total_release=>238}