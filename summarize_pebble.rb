reg =/
left_pebble\ =\ (\d*).*?
right_pebble\ =\ (\d*).*?
depth_pebble\ =\ (\d*).*?
depth2_pebble\ =\ (\d*).*?
left_tree_pebble\ =\ (\d*).*?
right_tree_pebble\ =\ (\d*).*?
depth_tree_pebble\ =\ (\d*).*?
depth_tree2_pebble\ =\ (\d*).*?
/xm

def fst(a, b, c, d)
  ary = [a, b, c, d].uniq.sort
  ary[0]
end

def snd(a, b, c, d)
  ary = [a, b, c, d].uniq.sort
  if ary.size >= 1
    ary[1]
  else
    nil
  end
end

def trd(a, b, c, d)
  ary = [a, b, c, d].uniq.sort
  if ary.size >= 2
    ary[2]
  else
    nil
  end
end

def fth(a, b, c, d)
  ary = [a, b, c, d].uniq.sort
  if ary.size >= 3
    ary[3]
  else
    nil
  end
end

# solo leader
def top(a, b, c, d)
  ary = [a, b, c, d].sort
  if ary.size >= 2 and ary[0] < ary[1]
    ary[0]
  else
    nil
  end
end
   
def count(tuples, pos)
  score = { fst: 0, snd: 0, trd: 0, fth: 0, top: 0, sethiullman_win: 0, sethiullman_lose: 0 }

  p tuples.size
  
  tuples.each_with_index { |tuple, idx|
    (a, b, c, d, a_, b_, c_, d_) = tuple
    if tuple[pos] == fst(a, b, c, d)
      score[:fst] += 1
    elsif tuple[pos] == snd(a, b, c, d)
      score[:snd] += 1
    elsif tuple[pos] == trd(a, b, c, d)
      score[:trd] += 1
    elsif tuple[pos] == fth(a, b, c, d)
      score[:fth] += 1
    end

    if tuple[pos] == top(a, b, c, d)
      score[:top] += 1
    end

    if tuple[pos] > tuple[pos+4]
      score[:sethiullman_win] += 1
    elsif tuple[pos] < tuple[pos+4]
      score[:sethiullman_lose] += 1
    end
  }

  score
end

File.open("pebble_comparison2.log") {|f|
  s = f.read
  r = s.scan(reg)

  tuples = r.map { |x|
    x.map(&:to_i)
  }

  count_left = count(tuples, 0)
  p count_left
  
  count_right = count(tuples, 1)
  p count_right
  
  count_depth1 = count(tuples, 2)
  p count_depth1

  count_depth2 = count(tuples, 3)
  p count_depth2

  pp avg_repair.sum(0.0) / avg_repair.size

  avg_xor_f_repair = tuples.map{|tuple|
    tuple[2].to_f / tuple[0].to_f
  }

  pp avg_xor_f_repair.sum(0.0) / avg_xor_f_repair.size

  avg_xor_r_repair = tuples.map{|tuple|
    tuple[3].to_f / tuple[0].to_f
  }

  pp avg_xor_r_repair.sum(0.0) / avg_xor_r_repair.size    
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
