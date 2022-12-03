file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

priorities = {}
("a".."z").each_with_index do |l,i|
  priorities[l] = i+1
end
("A".."Z").each_with_index do |l,i|
  priorities[l] = i+27
end

result = 0

lines.each do |line|
  chars = line.split("")
  pocket1 = chars[0..chars.length/2-1]
  pocket2 = chars[chars.length/2..]

  common = pocket1&pocket2
  result += priorities[common.first]
end

puts result

result = 0
lines.each_slice(3) do |group|
  common = group[0].split("")&group[1].split("")&group[2].split("")

  result += priorities[common.first]
end

puts result

