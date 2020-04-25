// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// CPU and Kernel missing feature warnings to suppress.
#[derive(Default, Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct WarningsToSuppress
{
	/// Missing CPU features whose warnings should be supressed.
	///
	/// Current names (and typical warning messages) are:-
	///
	/// * `spectre_v2_google`: "Kernel has `spectre_v2=retpoline,google`; this is probably not the best choice".
	/// * `has_rep_movsb_stosb`: "Your CPU does not support the REP MOVSB and REP STOSB instructions, which are optimal for some memory moves and copies".
	/// * `has_prefetchw`: "Your CPU does not support the PRETFCHW instruction, which is optimal for some memory moves and copies".
	/// * `has_ss`: "Your CPU does not support self-snoop of the cache (which nearly all should), which is important for efficient cache management in this application".
	/// * `has_working_xsave`: "CPU architecture either lacks XSAVE support or the Linux kernel has disabled XSAVE support".
	/// * `has_invpcid`: "Your CPU does not support the INVPCID instruction, which is important for efficient mitigation of the Meltdown and Spectre security vulnerabilities".
	/// * `has_smap`: "Your CPU does not support the Supervisor-Mode Access Prevention (SMAP) instructions CLAC and STAC, which are important for securing modern Linux systems".
	pub suppress_warnings_for_cpu_features: HashSet<String>,

	/// Missing Kernel features whose warnings should be supressed.
	///
	/// Current names (and typical warning messages) are:-
	///
	/// * `hashdist`: Warnings about `hashdist=0`.
	/// * `noaliencache`: "Kernel has `noaliencache` enabled; this is likely to hurt performance".
	/// * `numa_zonelist_order`: "Kernel has `noaliencache` enabled; this is likely to hurt performance".
	/// * `skew_tick`: "Kernel should have `skew_tick=1` for maximum performance at the cost of power consumption".
	/// * `idle_poll`: "Warnings about idle!=poll".
	/// * `noxsaveopt`: "Kernel has `noxsaveopt` enabled; this is likely to hurt performance".
	pub suppress_warnings_for_kernel_features: HashSet<String>,

	/// Miscellaneous warnings which should be supressed.
	///
	/// Current names (and typical warning messages) are:-
	///
	/// * `too_many_shared_hyper_threads`: "There are more than 2 shared hyper threads".
	/// * `too_few_shared_hyper_threads`: "There is only 1 shared hyper thread (which will be shared with the master logical core and control threads)".
	/// * `overlapping_shared_hyper_threads`: "More than one (actually, {}) hyper threads of the group '{:?}' are present in the shared hyper threads".
	/// * `too_many_numa_nodes_shared_hyper_threads`: "More than one (actually, {:?}) NUMA nodes are present in the shared hyper threads".
	pub suppress_warnings_for_miscellany: HashSet<String>,
}

impl WarningsToSuppress
{
	#[inline(always)]
	pub(crate) fn cpu_warn<F: FnOnce() -> bool>(&self, name: &str, message: &str, true_if_should_not_warn: F)
	{
		if true_if_should_not_warn()
		{
			return
		}

		if self.suppress_warnings_for_cpu_features.contains(name)
		{
			return
		}

		LoggingConfiguration::warn(name, format!("{}", message))
	}

	#[inline(always)]
	pub(crate) fn kernel_warn<F: FnOnce() -> bool>(&self, name: &str, message: &str, true_if_should_not_warn: F)
	{
		if true_if_should_not_warn()
		{
			return
		}

		if self.suppress_warnings_for_kernel_features.contains(name)
		{
			return
		}

		LoggingConfiguration::warn(name, format!("{}", message))
	}

	/// Miscellany warn.
	#[inline(always)]
	pub fn miscellany_warn<F: FnOnce() -> bool>(&self, name: &str, message: &str, true_if_should_not_warn: F)
	{
		if true_if_should_not_warn()
		{
			return
		}

		if self.suppress_warnings_for_miscellany.contains(name)
		{
			return
		}

		LoggingConfiguration::warn(name, format!("{}", message))
	}

	#[inline(always)]
	pub(crate) fn kernel_warn_without_check(&self, name: &str, message: &str)
	{
		if self.suppress_warnings_for_kernel_features.contains(name)
		{
			return
		}

		LoggingConfiguration::warn(name, format!("{}", message))
	}

	// Development on Mac Pro `trash cans` at this time assumes at least Intel Ivy Bridge CPUs.
	#[inline(always)]
	pub(crate) fn performance_warnings_it_is_safe_to_assume_for_all_x86_64_cpu_architectures_as_of_q2_2018(&self, feature_information: &FeatureInfo, extended_function_information: &ExtendedFunctionInfo, extended_feature_information: &ExtendedFeatures)
	{
		self.cpu_warn("has_rep_movsb_stosb", "Your CPU does not support the REP MOVSB and REP STOSB instructions, which are optimal for some memory moves and copies", || extended_feature_information.has_rep_movsb_stosb());

		self.cpu_warn("has_prefetchw", "Your CPU does not support the PRETFCHW instruction, which is optimal for some memory moves and copies", || extended_function_information.has_prefetchw());

		self.cpu_warn("has_ss", "Your CPU does not support self-snoop of the cache (which nearly all should), which is important for efficient cache management in this application", || feature_information.has_ss());

		self.cpu_warn("has_working_xsave", "CPU architecture either lacks XSAVE support or the Linux kernel has disabled XSAVE support", || feature_information.has_xsave() && feature_information.has_oxsave())
	}

	#[inline(always)]
	pub(crate) fn performance_warnings_for_new_features(&self, feature_information: &FeatureInfo, _extended_function_information: &ExtendedFunctionInfo, extended_feature_information: &ExtendedFeatures)
	{
		self.cpu_warn("has_invpcid", "Your CPU does not support the INVPCID instruction, which is important for efficient mitigation of the Meltdown and Spectre security vulnerabilities", || feature_information.has_xsave() && extended_feature_information.has_invpcid())
	}

	#[inline(always)]
	pub(crate) fn security_warnings_for_new_features(&self, _feature_information: &FeatureInfo, _extended_function_information: &ExtendedFunctionInfo, extended_feature_information: &ExtendedFeatures)
	{
		self.cpu_warn("has_smap", "Your CPU does not support the Supervisor-Mode Access Prevention (SMAP) instructions CLAC and STAC, which are important for securing modern Linux systems", || extended_feature_information.has_smap())
	}
}
