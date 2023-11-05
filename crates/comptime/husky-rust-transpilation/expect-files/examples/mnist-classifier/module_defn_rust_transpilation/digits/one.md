"fn!one_fermi_match{\n    fermi_match(major_concave_components, vec![downmost, upmost, hat]);\n}\npub!fn!upmost{\n    let!dp=cc.displacement();require!(dp.y>0)\n    dp.y;\n}\npub!fn!downmost{\n    let!dp=cc.displacement();require!(dp.y<=0)\n    -cc.end().y;\n}\npub!fn!hat{\n    let!dp=cc.displacement();require!(dp.y<0)require!(dp.x<0)\n    -dp.y-dp.x;\n}\n"