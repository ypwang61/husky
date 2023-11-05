"fn!left_components{\n    fermi_match(major_concave_components, vec![left_coordinate_max, left_coordinate_max]);\n}\npub!fn!left_coordinate_max{\n    cc.relative_bounding_box.xmax();\n}\nfn!components_max_downwards{\n    fermi_match(major_concave_components, vec![displacement_downwards]);\n}\nfn!components_max_heights{\n    fermi_match(major_concave_components, vec![cc_box_heights]);\n}\nfn!is_four{require!(matches!)require!(matches!)\n    let!eff_holes=major_connected_component.eff_holes;require!(matches!)\n    let!down_match=components_max_downwards.matches[0];require!(matches!)\n    let!down_match_dp_y=down_match.unwrap().displacement().y;\n    let!higher_excess=major_connected_component.upper_mass-major_connected_component.lower_mass;require!(higher_excess>7)    if!matches!{require!(major_concave_components.ilen()>=2)\n        let!four_match_refine_result=components_max_heights.matches[0];require!(matches!)require!(components_max_heights.norm<1)\n        let!higher_excess=major_connected_component.upper_mass-major_connected_component.lower_mass;\n        let!upper_arc=components_max_heights.matches[0];require!(matches!)require!(upper_arc.unwrap().displacement().y>0)require!(upper_arc.unwrap().angle_change<-110)require!(components_max_heights.norm<9)\n        let!a=major_connected_component.top_k_row_right_mass_sum(3);require!(a<22)require!(a>9)return!(OneVsAll::Yes)\n    }\n;\n    OneVsAll::Yes;\n}\npub!fn!displacement_downwards{\n    let!dp=cc.displacement();require!(dp.y<0)\n    dp.y;\n}\npub!fn!cc_box_heights{\n    let!dp=cc.displacement();require!(dp.y>0)require!(cc.relative_bounding_box.ymin()>0.4)\n    cc.relative_bounding_box.ymin();\n}\n"