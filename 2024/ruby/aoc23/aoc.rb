require "set"

file = File.open(ARGV[0])
data = file.readlines.map(&:chomp).map{|s| s.split("-")}

connects = {}
data.each do |d|
  connects[d[0]] ||= []
  connects[d[0]] << d[1]
  connects[d[1]] ||= []
  connects[d[1]] << d[0]
end

variants = []
connects.each do |k,v|
  v.uniq.combination(2).each do |combi|
    variants << (combi + [k]).sort.join(",")
  end
end

puts "Part1: #{variants.tally.select{|k,v| k =~ /t([a-z])/ && v == 3}.size}"

variants = {}
connects.each do |k,v|
  length = v.uniq.size
  (2..length).to_a.each do |i|
    variants[i] ||= []
    v.uniq.combination(i).each do |combi|
      variants[i] << (combi + [k]).sort.join(",")
    end
  end
end

filtered = {}
variants.keys.each do |k|
  filtered[k] = variants[k].tally.select{|key,val| val == k.to_i+1}
  puts "Part2: #{filtered[k].keys.first}" if filtered[k].size == 1
end
