file = File.open(ARGV[0])
data = file.readlines.map(&:chomp).map do |l|
  parts = l.split(' ')
  target = parts[0][1..-2].gsub('#', '1').gsub('.', '0').to_i(2)
  buttons = parts[1..-2].map do |b|
    b[1..-2].split(',').map(&:to_i).sum do |bit|
      1 << (parts[0][1..-2].length - 1) - bit
    end
  end
  joltage = parts[-1][1..-2].split(',').map(&:to_i)
  [target, buttons, joltage, l]
end

puts data.inspect

def find(result, buttons, raw)
  variants = buttons.dup.map { |b| [b] }
  loop do
    new_variants = Set.new
    variants.each do |v|
      if result == v.inject(:^)
        puts "found: #{result} == #{v} -- #{raw}"
        return v.size
      end
      buttons.each do |b|
        new_variants << (v + [b]).sort
      end
    end
    variants = new_variants
  end
end

puts "Part 1: #{data.map { |d| find(d[0], d[1], d[3]) }.sum}"
