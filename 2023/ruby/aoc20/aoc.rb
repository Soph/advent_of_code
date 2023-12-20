file = File.open(ARGV[0])
modules = file.readlines.map(&:chomp).map{|line| a = line.split(" -> "); [a[0], a[1].split(", ")]}

mapping = {}

broadcaster = nil
modules.each do |mod|
  if mod[0][0] == '%'
    mapping[mod[0][1..]] = [mod[0][0], false, mod[1]]    
  elsif mod[0][0] == '&'
    mapping[mod[0][1..]] = [mod[0][0], {}, mod[1]]
  elsif mod[0] == 'broadcaster'
    mapping[mod[0]] = ['broadcaster', false, mod[1]]
  end
end

puts mapping.inspect
# map inputs
outputs = []
mapping.each do |key, value|
  value[2].each do |output|
    if mapping[output].nil?
      outputs << output
    else
      if mapping[output][1].is_a?(Hash)
        mapping[output][1][key] = false
      end
    end
  end
end

outputs.each do |output|
  mapping[output] = [nil, {}, []]
end

puts mapping.inspect

pulses = []
count = {true => 0, false => 0}
puts pulses.inspect

puts "Running now"
1000.times do |i|
  pulses << ["button", false, "broadcaster"]
  while !pulses.empty?
    pulse = pulses.shift
    #puts pulse.inspect
    from = pulse[0]
    signal = pulse[1]
    target = pulse[2]
    count[signal] += 1
    if mapping[target][0] == '%' && !signal
      mapping[target][1] = !mapping[target][1]
      mapping[target][2].each do |new_target|
        pulses << [target, mapping[target][1], new_target]
      end
    elsif mapping[target][0] == '&'
      mapping[target][1][from] = signal
      uniq = mapping[target][1].values.uniq
      if uniq[0] == true && uniq.size == 1
        mapping[target][2].each do |new_target|
          pulses << [target, false, new_target]
        end
      else
        mapping[target][2].each do |new_target|
          pulses << [target, true, new_target]
        end
      end
    elsif mapping[target][0] == 'broadcaster'
      mapping[target][2].each do |new_target|
        pulses << [target, false, new_target]
      end
    end
  end
  #puts "----- loop #{i+1} finished ------"
end
puts count
puts "Part1: #{count[true] * count[false]}"



