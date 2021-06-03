reg =/
num_of_copies\ =\ (\d*).*?
left_pebble\ =\ (\d*).*?
right_pebble\ =\ (\d*).*?
depth_pebble\ =\ (\d*).*?
depth2_pebble\ =\ (\d*).*?
/xm

File.open("pebble_comparison2.log") {|f|
  s = f.read
  r = s.scan(reg)

  tuples = r.map { |x|
    x.map(&:to_i)
  }

  avg = tuples.map{|tuple|
    tuple[4].to_f / tuple[0].to_f
  }

  pp avg.sum(0.0) / avg.size
}

=begin
p fst(0, 1, 2)
p snd(0, 1, 2)
p trd(0, 1, 2)
p fst(0, 0, 0)
p snd(0, 0, 0)
p trd(0, 0, 0)
p fst(5, 0, 0)
p snd(5, 0, 0)
p trd(5, 0, 0)
p fst(5, 2, 0)
p snd(5, 2, 0)
p trd(5, 2, 0)
=end
