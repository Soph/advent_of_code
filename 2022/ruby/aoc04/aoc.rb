file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)


count = 0

lines.each do |line|
  assigments = line.split(",")
  left = assigments[0].split("-")
  right = assigments[1].split("-")

  if ((left[0]..left[1]).to_a & (right[0]..right[1]).to_a == (left[0]..left[1]).to_a) || ((left[0]..left[1]).to_a & (right[0]..right[1]).to_a == (right[0]..right[1]).to_a)
    count += 1
  end
end

puts "Result1: #{count}"

count = 0

lines.each do |line|
  assigments = line.split(",")
  left = assigments[0].split("-")
  right = assigments[1].split("-")

  if ((left[0]..left[1]).to_a & (right[0]..right[1]).to_a).size != 0
    count += 1
  end
end

puts "Result2: #{count}"

