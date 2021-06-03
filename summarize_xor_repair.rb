reg = /
original_slp.*?\ =\ (\d*).*?
defs_repair.*?\ =\ (\d*).*?
defs_xor_repair_f.*?\ =\ (\d*).*?
defs_xor_repair_r.*?\ =\ (\d*)
/xm

def fst(a, b, c)
  ary = [a, b, c].uniq.sort
  ary[0]
end

def snd(a, b, c)
  ary = [a, b, c].uniq.sort
  if ary.size >= 1
    ary[1]
  else
    nil
  end
end

def trd(a, b, c)
  ary = [a, b, c].uniq.sort
  if ary.size >= 2
    ary[2]
  else
    nil
  end
end

# solo leader
def top(a, b, c)
  ary = [a, b, c].sort
  if ary.size >= 2 and ary[0] < ary[1]
    ary[0]
  else
    nil
  end
end

def count(tuples, pos)
  score = { fst: 0, snd: 0, trd: 0, top: 0 }

  p tuples.size
  
  tuples.each { |tuple|
    _, a, b, c = tuple
    if tuple[pos] == fst(a, b, c)
      score[:fst] += 1
    elsif tuple[pos] == snd(a, b, c)
      score[:snd] += 1
    elsif tuple[pos] == trd(a, b, c)
      score[:trd] += 1
    end

    if tuple[pos] == top(a, b, c)
      score[:top] += 1
    end
  }

  score
end

File.open("xor_repair_comparison.log") {|f|
  s = f.read
  r = s.scan(reg)

  tuples = r.map { |x|
    x.map(&:to_i)
  }

  pp tuples.sort_by{ |x|
    [x[1] / x[0], x[2] / x[0], x[3] / x[0]].max
  }
  
  orig_max = tuples.map{|tuple| tuple[0]}.max
  idx_max = tuples.index{|tuple| tuple[0] == orig_max}
  
  pp [idx_max, tuples[idx_max]]

  minratioseq = tuples.map{|tuple|
    orig = tuple[0].to_f
    [tuple[1] / orig, tuple[2] / orig, tuple[3] / orig].min
  }
  minratio_idx = minratioseq.index(minratioseq.min)

  pp [minratio_idx, tuples[minratio_idx]]

  maxratioseq = tuples.map{|tuple|
    orig = tuple[0].to_f
    [tuple[1] / orig, tuple[2] / orig, tuple[3] / orig].max
  }
  maxratio = maxratioseq.select{|v| v < 1}.max
  maxratio_idx = maxratioseq.index(maxratio)

  pp [maxratio_idx, tuples[maxratio_idx]]
  
  
  count_triv = count(tuples, 1)
  p count_triv
  
  count_dist = count(tuples, 2)
  p count_dist
  
  count_long = count(tuples, 3)
  p count_long

  avg_repair = tuples.map{|tuple|
    tuple[1].to_f / tuple[0].to_f
  }

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
