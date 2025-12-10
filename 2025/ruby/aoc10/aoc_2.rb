file = File.open(ARGV[0])
data = file.readlines.map(&:chomp).map do |l|
  parts = l.split(' ')
  target = parts[0][1..-2].gsub('#', '1').gsub('.', '0').to_i(2)
  buttons = parts[1..-2].map do |b|
    b[1..-2].split(',').map(&:to_i)
  end
  joltage = parts[-1][1..-2].split(',').map(&:to_i)
  [target, buttons, joltage, l]
end

def prefill(data, result, count, size)
  removed = Set.new
  values = Set.new
  values << Array.new(size, 0)
  count.times do
    new_values = Set.new
    values.each do |v|
      data.each do |d|
        new_v = v.dup
        d.each do |n|
          new_v[n] += 1
        end
        next if removed.include?(new_v)

        if result.each_with_index.any? { |_k, i| new_v[i] > result[i] } # at least one value is already to high
          removed << new_v
          next
        end

        if values.include?(new_v) # we had the same state the round before, with less cost
          removed << new_v
          next
        end
        new_values << new_v
      end
    end
    values = new_values
  end
  values
end

sum = 0
data.each do |d|
  variants = prefill(d[1], d[2], d[2].max, d[2].size)
  puts "Build prefill for #{d[3]}, found #{variants.size}"
  i = d[2].max
  loop do
    found = false
    new_variants = Set.new
    variants.each do |variant|
      if variant == d[2]
        puts "Found #{i} for #{d[3]}"
        sum += i
        found = true
        break
      end
      d[1].each do |button|
        new_variant = variant.dup
        button.each do |digit|
          new_variant[digit] += 1
        end
        new_variants << new_variant
      end
    end
    break if found

    i += 1
    variants = new_variants
  end
end

puts sum
