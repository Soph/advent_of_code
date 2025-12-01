file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

dial = 50
zeros = 0

lines.each do |line|
  if line[0] == 'R'
    dial += line[1..].to_i
  else
    dial -= line[1..].to_i
  end
  dial %= 100
  zeros += 1 if dial.zero?
end

puts "Part 1: #{zeros}"

dial = 50
zeros = 0

lines.each do |line|
  mov = line[1..].to_i
  # zeros += mov / 100
  before = dial
  if line[0] == 'R'
    dial += mov
  else
    dial -= mov
  end
  before_mod = dial
  zeros += before_mod.abs / 100
  dial %= 100

  puts "#{before} #{line[0] == 'R' ? '+' : '-'} #{line[1..]} -> #{dial} --> #{zeros}" # if zeros - before_zeros > 0
end

puts "Part 2: #{zeros}"
