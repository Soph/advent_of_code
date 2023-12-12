file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

records = lines.map {|line| line.split(" ") }
records = records.map {|record| [record[0], record[1].split(",").map(&:to_i)]}

puts records.inspect

sum = 0
records.each_with_index do |record, i|
  puts i
  variants = [record[0]]
  while true do
    new_variants = []
    found = false
    variants.each do |variant|
      found = true if variant =~ /\?/
      new_variants << variant.sub("?", '#')
      new_variants << variant.sub("?", '.')
    end
    break unless found
    
    variants = new_variants.uniq
  end
  matches = variants.select{|variant| variant.chars.chunk{|y| y}.map{|y, ys| [y, ys.length]}.select{|k,v| k == '#'}.map{|k| k[1]} == record[1] }
  #puts "Record: #{record[0]} - #{record[1]}"
  #puts matches.inspect
  sum += variants.select{|variant| variant.chars.chunk{|y| y}.map{|y, ys| [y, ys.length]}.select{|k,v| k == '#'}.map{|k| k[1]} == record[1] }.size
end

puts "Part1: #{sum}"
