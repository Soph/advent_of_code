file = File.open(ARGV[0])
data = file.readlines.map(&:chomp).map do |l|
  parts = l.split(' ')
  target = parts[0][1..-2].gsub('#', '1').gsub('.', '0').to_i(2)
  buttons = parts[1..-2].map do |b|
    b[1..-2].split(',').map(&:to_i)
  end
  joltage = parts[-1][1..-2].split(',').map(&:to_i)
  joltage = Hash[(0...joltage.size).zip joltage]
  [target, buttons, joltage, l]
end

# puts data.inspect

def find(result, buttons, raw)
  variants = buttons.map { |b| b.tally }
  i = 0
  loop do
    new_variants = Set.new
    i += 1
    variants.each do |v|
      if result == v
        puts "found #{i}: #{result} == #{v} -- #{raw}"
        return i
      end
      next if v.any? { |key, _value| result[key] < v[key] } # we are bigger then needed already

      buttons.each do |b|
        new_v = v.merge(b.tally) { |_, val1, val2| val1 + val2 }
        new_variants << new_v
      end
    end
    variants = new_variants
  end
end

puts "Part 1: #{data.map { |d| find(d[2], d[1], d[3]) }.sum}"
