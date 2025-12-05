parts = File.read(ARGV[0]).split("\n\n")

ranges = parts[0].split("\n").map { |p| p.split('-').map(&:to_i) }
count = 0
parts[1].split("\n").map(&:to_i).each do |ingredient|
  ranges.each do |range|
    # puts "#{ingredient} >= #{range[0]} && #{ingredient} <= #{range[1]}"
    if ingredient >= range[0] && ingredient <= range[1]
      count += 1
      break
    end
  end
end

puts "Part 1: #{count}"

loop do
  new_ranges = Set.new
  ranges.each do |range|
    found = false
    ranges.each do |range_a|
      next if range == range_a

      start_included = false
      end_included = false

      start_included = true if range[0] >= range_a[0] && range[0] <= range_a[1]
      end_included = true if range[1] >= range_a[0] && range[1] <= range_a[1]
      next if !start_included && !end_included

      if start_included && end_included
        new_ranges << range_a
      elsif start_included
        new_ranges << [range_a[0], range[1]]
      elsif end_included
        new_ranges << [range[0], range_a[1]]
      end
      found = true
      break
    end
    new_ranges << range unless found # keep the ones that can't be normalized anymore
  end

  break if new_ranges == ranges

  ranges = new_ranges
end

puts "Part 2: #{ranges.map { |range| (range[0]..range[1]).size }.sum}"
